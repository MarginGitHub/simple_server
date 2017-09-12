use beans::user::User;
use diesel::insert;
use diesel::SqliteConnection;

pub mod schema;



pub fn save_user(conn: &SqliteConnection, u: &User) {
    use diesel::ExecuteDsl;
    let ret = insert(u).into(schema::user::table).execute(conn);
    match ret {
        Ok(ret) => println!("{:?}", ret),
        Err(e) => println!("{:?}", e),
    }
}

pub fn get_user(conn: &SqliteConnection, u: User) -> Option<User> {
    use self::schema::user::dsl::*;
    use diesel::*;
    user.filter(uid.eq(20)).limit(5).load::<User>(conn);
    Some(u)
}