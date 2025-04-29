-- Your SQL goes here
CREATE TABLE IF NOT EXISTS groups (
    id serial not null,
    group_name text not null,
    constraint pk_group_id primary key(id)
);

CREATE TABLE IF NOT EXISTS user_groups (
    id serial not null,
    user_id int not null,
    group_id int not null ,
    constraint pk_usergroup_id primary key(id),
    constraint fk_userid foreign key (user_id) references users(id),
    constraint fk_groupid foreign key (group_id) references groups(id)
);
