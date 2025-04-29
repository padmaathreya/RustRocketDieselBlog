
use crate::schema::{ users,groups,user_groups};
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

//sub task 1

#[derive(Debug,Identifiable,Queryable,Serialize, Deserialize,Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize,Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize,Serialize)]
pub struct NewUserInput<'a> {
    pub username: &'a str,
    pub first_name:  &'a str,
    pub last_name:  &'a str,
    pub group_ids: Vec<i32>,
}

#[derive(Debug,Queryable,Serialize, Deserialize,Clone)]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: i32,
    pub group_name: String
}
#[derive(Debug,Insertable,Serialize, Deserialize,Clone)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    pub group_name: String
}
#[derive(Debug,Queryable,Serialize, Deserialize,Clone)]
#[diesel(table_name = user_groups)]
pub struct UserGroup {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize,Debug)]
#[diesel(table_name = user_groups)]
pub struct NewUserGroup {
    pub user_id: i32,
    pub group_id: i32
}

#[derive(Debug,Serialize, Deserialize,Clone,Queryable)]
pub struct UserList {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub group_ids: Vec<i32>
   // pub group_ids: Option<Vec<i32>>
}