-- Your SQL goes here
-- Your SQL goes here
--Rust task: Simple blog API
/* The schema design must be something like this:
1. users: id (PK), username (unique), firstname, lastname
2. posts: id (PK), createdby (FK, users), title, body */

CREATE TABLE IF NOT EXISTS users (
    id serial not null,
    username text not null unique,
    first_name text not null,
    last_name text not null,
    constraint pk_user_id primary key(id)
);

-- Your SQL goes here
CREATE TABLE if not EXISTS posts (
  id SERIAL,
  created_by int not null,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE,
   constraint pk_post_id primary key(id),
 constraint fk_user_id foreign key(created_by) references users(id)
);