-- Your SQL goes here

CREATE TABLE posts_tags (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag TEXT NOT NULL
);