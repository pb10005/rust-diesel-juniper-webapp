-- Your SQL goes here
CREATE TABLE users (
  id   SERIAL  PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE posts (
  id    SERIAL PRIMARY KEY,
  user_id SERIAL REFERENCES users(id),
  title VARCHAR NOT NULL,
  body  VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE VIEW post_details AS
SELECT 
p.id,
u.name,
p.title,
p.body,
p.created_at
FROM posts p
INNER JOIN users u
on p.user_id = u.id
;