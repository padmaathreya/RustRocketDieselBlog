extern crate rocket;
use rocket::{ launch, routes };
use rocket_dyn_templates::{ Template };
pub mod services;
pub mod models;
pub mod schema;
/*

 http://127.0.0.1:8000/user
{
    "username": "firstlast3",
    "first_name": "user 3",
    "last_name": "last 3"
}
    http://127.0.0.1:8000/post
{
    "title": "The Statesman",
    "body": "The new age politician"
}
    
*/

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    let bind = dotenvy::var("BIND").expect("BIND var is not available in .env"); //localhost:xxxx
    let cust_port: u16 = bind.parse().expect("should be a valid port");
    rocket
        ::custom(rocket::Config {
            port: cust_port,
            ..rocket::Config::default()
        })
        .mount("/", routes![services::post::create_post]) //subtask 1
        .mount("/", routes![services::post::list_posts]) //sub task 1
        .mount("/", routes![services::post::list_posts_with_tags]) //sub task 2
        .mount("/", routes![services::post::list_posts3]) //subtask3
        .mount("/", routes![services::user::create_user])//subtask 1
        .mount("/", routes![services::user::list_users])//subtask 1
        .mount("/", routes![services::user::get_users])//subtask 1
        .attach(Template::fairing())
}
