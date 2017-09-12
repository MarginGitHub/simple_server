pub use db;

mod posts;
mod gets;

#[derive(Debug, Serialize)]
pub struct Reply<'a, T> {
    code: i32,
    message: &'a str,
    data: Option<T>,
}

impl<'a, T> Reply<'a, T> {
    // add code here
    pub fn new(code: i32, message: &'a str, data: Option<T>) -> Self {
        Reply{
            code,
            message,
            data
        }
    }
}

pub fn start() {
    ::rocket::ignite()
        .mount("/", posts::get_routes())
        .launch();
}