extern crate diesel;
extern crate rocket;

use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::{ Array, Integer, Nullable };
use rocket::http::Status;
use rocket::response::{ status::Created, Debug };
use rocket::serde::{ json::Json };
use rocket::{ get, post };
use rocket_dyn_templates::{ context, Template };

use crate::models::user::{ NewUserGroup, NewUserInput, User, UserList };
use crate::models::{ user::NewUser };
use crate::models::paginate::{ PaginatedResponse, PaginationMeta };

use crate::schema::user_groups::dsl::user_groups;
use crate::schema::users::dsl::users;
use crate::services::db::establish_connection_pg;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

diesel::sql_function! {
    #[sql_name = "ARRAY_AGG"]
    fn array_agg(x: Integer) -> Array<Integer>;
}

#[post("/user", format = "json", data = "<user_input>")]
pub fn create_user(user_input: Json<NewUserInput>) -> Result<Created<Json<NewUserInput>>, Status> {
    let connection = &mut establish_connection_pg();

    let new_user = NewUser {
        first_name: user_input.first_name.to_string(),
        last_name: user_input.last_name.to_string(),
        username: user_input.username.to_string(),
    };
    let result = diesel::insert_into(users).values(&new_user).get_result::<User>(connection);
    if result.is_err() {
        return Err(Status::NoContent);
    }
    let new_groups: Vec<NewUserGroup> = match result {
        Ok(val) =>
            user_input.group_ids
                .iter()
                .map(|group_id| NewUserGroup { user_id: val.id, group_id: *group_id })
                .collect(),
        Err(e) => {
            return Err(Status::NoContent);
        }
    };

    let result = diesel::insert_into(user_groups).values(&new_groups).execute(connection);
    if result.is_err() {
        println!("Unable to insert into user_groups");
        return Err(Status::NoContent);
    }

    Ok(Created::new("/").body(user_input))
}

#[get("/users")]
pub fn list_users() -> Template {
    use crate::models::user::User;
    let connection = &mut establish_connection_pg();
    let results = users.load::<User>(connection).expect("Error loading users");
    println!("{:?}", &results);
    Template::render("users", context! { users: &results, count: results.len() })
}

//http://localhost:8000/users?page=1&limit=2&search=test
#[get("/userss?<page>&<limit>&<search>")]
pub fn get_users(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>
) -> Json<PaginatedResponse<User>> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let search = search.unwrap_or_default();

    let connection = &mut establish_connection_pg();
    let results = users.load::<User>(connection).expect("Error loading users");

    // Filter
    let filtered: Vec<User> = results
        .into_iter()
        .filter(|u| u.username.to_lowercase().contains(&search.to_lowercase()))
        .collect();

    let total_docs = filtered.len() as u32;
    let total_pages = (total_docs + limit - 1) / limit;
    let start = ((page - 1) * limit) as usize;
    let end = (start + (limit as usize)).min(filtered.len());

    let records = filtered[start..end].to_vec();

    let meta = PaginationMeta {
        current_page: page,
        per_page: limit,
        from: ((start as u32) + 1).min(total_docs),
        to: (end as u32).min(total_docs),
        total_pages,
        total_docs,
    };

    Json(PaginatedResponse { records, meta })
}


#[get("/usergroups?<page>&<limit>&<search>")]
pub fn get_userwithgroupids(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>
) -> Result<Json<PaginatedResponse<UserList>>, Status> {
    use crate::schema::users::{ self, dsl::* };
    use crate::schema::user_groups::{ self, dsl::* };

    let connection = &mut establish_connection_pg();

    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;

    let search_term = search.unwrap_or_default();
    let search_pattern = format!("%{}%", search_term.to_lowercase());

    let total_docs: i64 = users
        .filter(username.ilike(&search_pattern))
        .count()
        .get_result(connection)
        .unwrap_or(0);

    let total_pages = ((total_docs + (limit as i64) - 1) / (limit as i64)).max(1);
   
    let results = users
    .left_join(user_groups.on(user_id.eq(users::id)))
    .filter(username.ilike(&search_pattern))
    .select((
        users::id,
        users::username,
        users::first_name,
        users::last_name,
        sql::<Array<Integer>>(
            "COALESCE(ARRAY_REMOVE(ARRAY_AGG(user_groups.group_id), NULL), '{}')"
        ),
    ))
    .group_by((
        users::id,
        users::username,
        users::first_name,
        users::last_name,
    ))
    .order(users::id)
    .limit(limit.into())
    .offset(offset.into())
    .load::<UserList>(connection);

    println!("{:?}", &results);

   

    let records: Vec<UserList> = match results {
        Ok(val) => val,
        Err(e) => {
            println!("{}",e);
            return Err(Status::NoContent);
        }
    };

    let meta = PaginationMeta {
        current_page: page,
        per_page: limit,
        from: offset + 1,
        to: offset + (records.len() as u32),
        total_pages: total_pages as u32,
        total_docs: total_docs as u32,
    };

    Ok(Json(PaginatedResponse { records, meta }))
}
