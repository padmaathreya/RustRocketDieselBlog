
use crate::schema::{posts, posts_tags};
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

//sub task 1


#[derive(Queryable, Serialize, Deserialize,Clone,Debug,Identifiable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub created_by: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable,Deserialize,Serialize, Clone, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub created_by: Option<i32>,
    pub title: &'a str,
    pub body:  &'a str,
}

//sub task 2

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
#[diesel(table_name = posts_tags)]
pub struct PostTag {
    pub id: i32,
    pub post_id: i32,
    pub tag: String,
}

#[derive(Deserialize,Serialize)]
pub struct NewPostInput<'a> {
    pub created_by: Option<i32>,
    pub title:  &'a str,
    pub body:  &'a str,
    pub tags: Vec< &'a str,>,
}

#[derive(Insertable)]
#[table_name = "posts_tags"]
pub struct NewPostTag<'a> {
    pub post_id: i32,
    pub tag: &'a str,
}


#[derive(Serialize)]
pub struct PostWithTags {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}
//Sub task 3

#[derive(Serialize)]
pub struct CreatedBy {
    pub user_id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_by: Option<CreatedBy>,
}
