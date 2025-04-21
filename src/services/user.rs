extern crate diesel;
extern crate rocket;

use diesel::prelude::*;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::response::{ status::Created, Debug };
use rocket::serde::{ json::Json };
use rocket::{ get, post };
use rocket_dyn_templates::{ context, Template };


use crate::models::db_tables::User;
use crate::models::{db_tables::NewUser};
use crate::models::paginate::{PaginatedResponse, PaginationMeta};

use crate::schema::users::dsl::users;
use crate::services::db::establish_connection_pg;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Result<Created<Json<NewUser>>, Status> {
    let connection = &mut establish_connection_pg();

    let new_user = NewUser {
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        username: user.username.to_string(),
    };
    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(connection);
    if result.is_err() {
        return Err(Status::NoContent);
    }
    Ok(Created::new("/").body(user))
}

#[get("/users")]
pub fn list_users() -> Template {
    use crate::models::db_tables::User;
    let connection = &mut establish_connection_pg();
    let results = users
        .load::<User>(connection)
        .expect("Error loading users");
    println!("{:?}", &results);
    Template::render("users", context! { users: &results, count: results.len() })
}
//http://localhost:8000/users?page=1&limit=2&search=test
#[get("/userss?<page>&<limit>&<search>")]
pub fn get_users(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
) -> Json<PaginatedResponse<User>> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let search = search.unwrap_or_default();

     let connection = &mut establish_connection_pg();
    let results = users
        .load::<User>(connection)
        .expect("Error loading users");

    // Filter
    let filtered: Vec<User> = results
        .into_iter()
        .filter(|u| u.username.to_lowercase().contains(&search.to_lowercase()))
        .collect();

    let total_docs = filtered.len() as u32;
    let total_pages = (total_docs + limit - 1) / limit;
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(filtered.len());

    let records = filtered[start..end].to_vec();

    let meta = PaginationMeta {
        current_page: page,
        per_page: limit,
        from: (start as u32 + 1).min(total_docs),
        to: (end as u32).min(total_docs),
        total_pages,
        total_docs,
    };

    Json(PaginatedResponse { records, meta })
}
