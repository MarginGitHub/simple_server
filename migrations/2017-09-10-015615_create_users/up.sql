-- Your SQL goes here

CREATE TABLE user (
  id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE,
  uid INTEGER NOT NULL,
  mobile TEXT NOT NULL,
  password TEXT NOT NULL,
  nick_name TEXT,
  head_img_url TEXT,
  last_login_time INTEGER
);

CREATE TABLE ids (
	id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE
);