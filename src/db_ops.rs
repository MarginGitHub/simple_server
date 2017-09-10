use rusqlite::{self, Connection};
use beans::*;

pub fn add_user(conn: &Connection, user: User)  {
	let sql = "insert into user(
					uid, nick_name, mobile, password, 
					head_img_url, last_login_time)
				values(?1, ?2, ?3, ?4, ?5, ?6)";
	let ret = conn.execute(sql, &[&user.uid, &user.nick_name, &user.mobile, 
		&user.password, &user.head_img_url, &user.last_login_time]);
	match ret {
	    Ok(_) => {},
	    Err(err) => println!("{:?}", err),
	}
}

pub fn query_last_user(conn: &Connection) -> Option<User> {
	let sql = "select * from user order by id desc limit 1";
	let result = conn.query_row(sql, &[], |row| {
		let id: isize = row.get(0);
		println!("{:?}", id);
		let user = User{
			uid: row.get(1),
			mobile: row.get(2),
			password: row.get(3),
			nick_name: row.get(4),
			head_img_url: row.get(5),
			last_login_time: row.get(6)
		};
		user
	});
	match result {
		Ok(u) => {
			Some(u)
		},
		Err(err) => {
			println!("{:?}", err);
			None
		}
	}
}

pub fn query_user_by_uid(conn: &Connection, uid: isize) -> RetType {
    unimplemented!();
}