use rocket::request::{Form};
use rocket::Route;
use rocket_contrib::Json;
use beans::register::Register;
use beans::login::Login;
use beans::user::User;
use net::Reply;

pub fn get_routes() -> Vec<Route> {
    routes![register, login]
}

#[post("/register", data="<data>")]
fn register(data: Form<Register>) -> Json<Reply<bool>> {
    let data = data.get();
    println!("\tmobile: {}, \n\tpassword: {}", 
                            data.mobile, data.password);
    Json(Reply::new(100, "成功", None))
}

#[post("/login", data="<data>")]
fn login(data: Form<Login>) -> Json<Reply<User>> {
    let login = data.get();
    if login.mobile == "18268854059" && login.password == "123456" {
        Json(
            Reply::new(100, "成功", Some(
                User::new(10, "18268854059".to_string(), "margin".to_string())
            ))
        )
    } else {
        Json(
            Reply::new(101, "失败,用户名或密码不正确", None)
        )
    }
}