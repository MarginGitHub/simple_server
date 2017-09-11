use rocket_codegen::*;
use serde::Serialize;

#[derive(Debug, FromForm)]
pub struct Register {
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct Login {
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub uid: i32,
    pub mobile: String,
    pub name: String,
}

impl User {
    // add code here
    #[inline]
    pub fn new(uid: i32, mobile: String, name: String) -> Self {
        User{uid, mobile, name}
    }
}