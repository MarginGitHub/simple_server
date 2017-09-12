-- Your SQL goes here

CREATE TABLE user (
  id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
  uid INTEGER NOT NULL,
  mobile TEXT NOT NULL,
  password TEXT NOT NULL,
  nick_name TEXT,
  avatar_url TEXT,
  last_login_time INTEGER
);