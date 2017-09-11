#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#![recursion_limit="128"]
#![feature(custom_attribute)]

pub extern crate rocket;
extern crate rocket_contrib;
#[macro_use] pub extern crate rocket_codegen;

extern crate serde;
#[macro_use] extern crate serde_derive;

pub extern crate diesel;
#[macro_use] extern crate diesel_codegen;

pub extern crate r2d2;
pub extern crate r2d2_diesel;

pub mod db;
pub mod net;
pub mod beans;

