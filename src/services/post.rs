extern crate diesel;
extern crate rocket;
use diesel::dsl::sql;
use diesel::prelude::*;

use rocket::http::Status;
use rocket::response::{ status::Created, Debug };
use rocket::serde::{ json::Json };
use rocket::{ get, post };

use crate::models::paginate::{ PaginatedResponse, PaginationMeta };
use crate::models::db_tables::*;

use crate::schema::posts::dsl::*;
use crate::schema::posts_tags::dsl::*;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

use super::db::establish_connection_pg;

//Task 1: create post

#[post("/post", format = "json", data = "<post_input>")]
pub fn create_post(
    post_input: Json<NewPostInput>
) -> Result<Created<Json<NewPostInput<'_>>>, Status> {
    use crate::schema::posts::dsl::posts;

    let connection = &mut establish_connection_pg();

    let new_post = NewPost {
        created_by: post_input.created_by,
        title: &post_input.title,
        body: &post_input.body,
    };

    let result = diesel::insert_into(posts).values(&new_post).get_result::<Post>(connection);

    let new_tags: Vec<NewPostTag> = match result {
        Ok(val) =>
            post_input.tags
                .iter()
                .map(|tag_c| NewPostTag { post_id: val.id, tag: tag_c })
                .collect(),
        Err(e) => {
            return Err(Status::NoContent);
        }
    };

    let result = diesel::insert_into(posts_tags).values(&new_tags).execute(connection);
    if result.is_err() {
        return Err(Status::NoContent);
    }
    Ok(Created::new("/").body(post_input))
}

//Task 1: list posts with pagination

#[get("/listposts1?<page>&<limit>&<search>")]
pub fn list_posts(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>
) -> Json<PaginatedResponse<Post>> {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(10);
    let search = search.unwrap_or_default();

    let connection = &mut establish_connection_pg();
    let results = posts.load::<Post>(connection).expect("Error loading posts");

    let filtered: Vec<Post> = results
        .into_iter()
        .filter(|u| u.title.to_lowercase().contains(&search.to_lowercase()))
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

use diesel::sql_types::{ Array, Text };

//sub task 2
#[get("/listpost2?<page>&<limit>&<search>")]
pub fn list_posts_with_tags(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>
) -> Json<PaginatedResponse<PostWithTags>> {
    use crate::schema::posts::{ self, dsl::* };

    let connection = &mut establish_connection_pg();

    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;

    let search_term = search.unwrap_or_default();
    let search_pattern = format!("%{}%", search_term.to_lowercase());

    let total_docs: i64 = posts
        .filter(title.ilike(&search_pattern))
        .count()
        .get_result(connection)
        .unwrap_or(0);

    let total_pages = ((total_docs + (limit as i64) - 1) / (limit as i64)).max(1);

    let results = posts
        .left_join(posts_tags.on(post_id.eq(posts::id)))
        .filter(title.ilike(&search_pattern))
        .select((
            posts::id,
            posts::title,
            posts::body,
            sql::<Array<Text>>(
                "ARRAY_AGG(posts_tags.tag) FILTER (WHERE posts_tags.tag IS NOT NULL)"
            ),
        ))
        .group_by(posts::id)
        .order(posts::id)
        .limit(limit.into())
        .offset(offset.into())
        .load::<(i32, String, String, Vec<String>)>(connection)
        .unwrap();

    let records: Vec<PostWithTags> = results
        .into_iter()
        .map(|(c_id, c_title, c_body, c_tags)| PostWithTags {
            id: c_id,
            title: c_title,
            body: c_body,
            tags: c_tags,
        })
        .collect();

    let meta = PaginationMeta {
        current_page: page,
        per_page: limit,
        from: offset + 1,
        to: offset + (records.len() as u32),
        total_pages: total_pages as u32,
        total_docs: total_docs as u32,
    };

    Json(PaginatedResponse { records, meta })
}

// SUB TASK 3

#[get("/listpost3?<page>&<limit>&<search>")]
pub async fn list_posts3(
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>
) -> Json<PaginatedResponse<PostResponse>> {
    use crate::schema::{ posts, users };

    let connection = &mut establish_connection_pg();

    let page: i64 = page.unwrap_or(1).max(1) as i64;
    let limit: i64 = limit.unwrap_or(10).min(100) as i64;
    let offset: i64 = (page - 1) * (limit as i64);

    // Base query with left join and optional search filter
    let mut base_query = posts::table
        .left_join(users::table.on(users::id.nullable().eq(posts::created_by)))
        .into_boxed();

    if let Some(search_text) = &search {
        let pattern = format!("%{}%", search_text);
        base_query = base_query.filter(
            posts::title.ilike(pattern.clone()).or(users::username.ilike(pattern.clone()))
        );
    }

    // Get paginated results
    let results: Vec<
        (i32, String, String, Option<i32>, Option<String>, Option<String>, Option<String>)
    > = base_query
        .select((
            posts::id,
            posts::title,
            posts::body,
            users::id.nullable(),
            users::username.nullable(),
            users::first_name.nullable(),
            users::last_name.nullable(),
        ))
        .limit(limit)
        .offset(offset)
        .load::<(i32, String, String, Option<i32>, Option<String>, Option<String>, Option<String>)>(
            connection
        )
        .expect("Error loading posts");

    println!("{:?}",  results.len());

    // Map results to response format
    let records: Vec<PostResponse> = results
        .into_iter()
        .map(|(id_c, title_c, body_c, user_id, username, first_name, last_name)| {
            PostResponse {
                id: id_c,
                title: title_c,
                body: body_c,
                created_by: user_id.map(|uid| CreatedBy {
                    user_id: uid,
                    username: username.unwrap_or_default(),
                    first_name: first_name.unwrap_or_default(),
                    last_name,
                }),
            }
        })
        .collect();

     println!("{:?}",  records.len());

    // Get total matching documents (single count query)
  let total_docs: i64 = records.len() as i64;

    // Calculate pagination metadata
    let total_pages = ((total_docs as f64) / (limit as f64)).ceil() as u32;
    let from = (if total_docs == 0 { 0 } else { offset + 1 }) as u32;
    let to = (offset + limit).min(total_docs) as u32;

    Json(PaginatedResponse {
        records,
        meta: PaginationMeta {
            current_page: page as u32,
            per_page: limit as u32,
            from,
            to,
            total_pages,
            total_docs: total_docs as u32,
        },
    })
}

// #[get("/listpost31?<page>&<limit>&<search>")]
// pub fn list_posts_with_createdby(
//     page: Option<u32>,
//     limit: Option<u32>,
//     search: Option<String>
// ) -> Json<PaginatedResponse<PostResponse>> {
//     use diesel::prelude::*;
//     use crate::schema::{ posts, users };

//     let connection = &mut establish_connection_pg();

//     let results = posts::table
//         .left_join(users::table.on(users::id.nullable().eq(posts::created_by)))
//         .select((
//             posts::id,
//             posts::title,
//             posts::body,
//             // user fields as Option
//             users::id.nullable(),
//             users::username.nullable(),
//             users::first_name.nullable(),
//             users::last_name.nullable(),
//         ))
//         .load::<(i32, String, String, Option<i32>, Option<String>, Option<String>, Option<String>)>(
//             connection
//         )
//         .unwrap();

//     let post_c: Vec<PostResponse> = results
//         .into_iter()
//         .map(|(id_c, title_c, body_c, user_id_c, username_c, first_name_c, last_name_c)| {
//             let created_by_c = user_id_c.map(|uid| CreatedBy {
//                 user_id: uid,
//                 username: username_c.unwrap_or_default(),
//                 first_name: first_name_c.unwrap_or_default(),
//                 last_name: last_name_c,
//             });
//             PostResponse {
//                 id: id_c,
//                 title: title_c,
//                 body: title_c,
//                 created_by: created_by_c,
//             }
//         })
//         .collect();

//     let meta = PaginationMeta {
//         current_page: page,
//         per_page: limit,
//         from: offset + 1,
//         to: offset + (records.len() as u32),
//         total_pages: total_pages as u32,
//         total_docs: total_docs as u32,
//     };

//     Json(PaginatedResponse { records, meta })
// }

// // other end points
// use rocket::response::status::Custom;
// use rocket_dyn_templates::{ context, Template };
// use super::error::ErrorResponse;
// use diesel::{ prelude::* };

// #[get("/posts1")]
// pub fn list_posts1() -> Template {
//     use crate::models::db_tables::Post;
//     use crate::schema::posts::dsl::posts;

//     let connection = &mut establish_connection_pg();
//     let results = posts.load::<Post>(connection).expect("Error loading posts");
//     Template::render("posts", context! { posts: &results, count: results.len() })
// }

// //http://localhost:9092/posts?page=1&per_page=2
// #[get("/posts2?<page>&<per_page>")]
// pub fn list_posts2(
//     page: Option<i64>,
//     per_page: Option<i64>
// ) -> Result<Json<Vec<Post>>, Custom<Json<ErrorResponse>>> {
//     let connection = &mut establish_connection_pg();

//     let page = page.unwrap_or(1);
//     let per_page = per_page.unwrap_or(10);

//     match get_paginated_posts(connection, page, per_page) {
//         Ok(postlist) => Ok(Json(postlist)),
//         Err(e) => Err(Custom(Status::InternalServerError, Json(ErrorResponse::from(e)))),
//     }
// }

// pub fn get_paginated_posts(
//     conn: &mut PgConnection,
//     page: i64,
//     per_page: i64
// ) -> Result<Vec<Post>, diesel::result::Error> {
//     use crate::schema::posts::dsl::*;
//     posts
//         .order(id)
//         .limit(per_page)
//         .offset((page - 1) * per_page)
//         .load::<Post>(conn)
// }
