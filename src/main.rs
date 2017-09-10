#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![recursion_limit="128"]
#![feature(custom_attribute)]


extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rusqlite;

extern crate r2d2;
extern crate r2d2_sqlite;


pub mod beans;
pub mod routes;
pub mod database;
pub mod db_ops;


fn main() {
	routes::launch();
}