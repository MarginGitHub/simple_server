use std::ops::Deref;
use std::env::var;

use rusqlite::Connection;

use r2d2::{self, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};



// type Pool = r2d2::Pool<SqliteConnectionManager<SqliteConnection>>;


// Connection request guard type: a wrapper around an r2d2 pooled connection.
// pub struct DbConn(pub PooledConnection<SqliteConnectionManager<SqliteConnection>>);
type Pool = r2d2::Pool<SqliteConnectionManager>;
#[derive(Debug)]
pub struct DbConn ( pub PooledConnection<SqliteConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_pool() -> Pool {
	let db_url = var("DATABASE_URL").unwrap();
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new(db_url);
    let pool_ret = r2d2::Pool::new(config, manager);
    match pool_ret {
        Ok(pool) => pool,
        Err(err) => {
        	panic!("{:?}", err);
        },
    }
}