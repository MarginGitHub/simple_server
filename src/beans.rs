use serde::*;

#[derive(Debug, Serialize)]
pub struct RepResult<T: Serialize> {
    code: u32,
    message: &'static str,
    data: T,
}

impl<T: Serialize> RepResult<T> {
    // add code here
    pub fn new(code: u32, msg: &'static str, data: T) -> Self {
    	RepResult {
    		code: code,
    		message: msg,
    		data: data
    	}
    }
}


#[derive(Debug, FromForm)]
pub struct Login {
    mobile: String,
    password: String
}

#[derive(Debug, FromForm)]
pub struct Register {
    pub mobile: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct RegisterResult {
    mobile: String,
    password: String
}

impl RegisterResult {
    // add code here
    pub fn new(m: String, p: String) -> Self {
    	RegisterResult{
    		mobile: m,
    		password: p
    	}
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    pub uid: isize,
    pub nick_name: Option<String>,
    pub mobile: String,
    pub password: String,
    pub head_img_url: Option<String>,
    pub last_login_time: Option<isize>,
}