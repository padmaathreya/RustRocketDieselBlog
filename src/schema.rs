// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        group_name -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        created_by -> Nullable<Int4>,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    posts_tags (id) {
        id -> Int4,
        post_id -> Int4,
        tag -> Text,
    }
}

diesel::table! {
    user_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (post_id));
diesel::joinable!(user_groups -> groups (group_id));
diesel::joinable!(user_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    posts,
    posts_tags,
    user_groups,
    users,
);
