use rocket;
use rocket::request::*;
use rocket_contrib::Json;


use beans::*;
use database::*;
use db_ops::*;

#[post("/v1/register", data="<info>")]
fn register(info: Form<Register>, conn: DbConn) -> Json<RepResult<RegisterResult>> {

	match info.into_inner() {
	    Register {mobile, password} => {
	    	let mut user = User {
	    		uid: 1,
	    		nick_name: None,
	    		mobile: mobile.clone(),
	    		password: password.clone(),
	    		head_img_url: None,
	    		last_login_time: None
	    	};
	    	match query_last_user(&conn) {
	    		Ok(u) => {
	    			user.uid = u.uid + 1;
	    			println!("uid: {}", user.uid);
	    		},
	    		Err(err) => {
	    			println!("{:?}", err);
	    		}
	    	}
	    	add_user(&conn, user);
	    	Json (RepResult::new(100 as u32, 
	    		"successful", 
	    		RegisterResult::new(mobile.clone(), password.clone())))
	    },
	}
}


pub fn launch() {
    rocket::ignite()
    	.manage(init_pool())
    	.mount("/", routes![register]).launch();
}