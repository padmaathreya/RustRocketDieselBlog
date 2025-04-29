extern crate diesel;
extern crate rocket;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::{ status::Created, Debug };
use rocket::serde::{ json::Json };
use rocket::{ get, post };
use rocket_dyn_templates::{ context, Template };

use crate::models::user::{Group};
use crate::models::{user::NewGroup };

use crate::schema::groups::dsl::groups;
use crate::services::db::establish_connection_pg;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/group", format = "json", data = "<group>")]
pub fn create_group(group: Json<NewGroup>) -> Result<Created<Json<NewGroup>>, Status> {
    let connection = &mut establish_connection_pg();

    let new_group = NewGroup {
        group_name: group.group_name.to_string(),
    };
    let result = diesel::insert_into(groups).values(&new_group).execute(connection);
    if result.is_err() {
        return Err(Status::NoContent);
    }
    Ok(Created::new("/").body(group))
}


#[get("/groups")]
pub fn list_groups() -> Template {
    use crate::models::user::Group;
    let connection = &mut establish_connection_pg();
    let results = groups.load::<Group>(connection).expect("Error loading groups");
    println!("{:?}", &results);
    Template::render("groups", context! { groups: &results, count: results.len() })
}