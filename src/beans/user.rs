use db::schema::user;

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name="user"]
pub struct User {
    pub uid: i32,
    pub mobile: String,
    pub password: String,
    pub nick_name: Option<String>,
    pub avatar_url: Option<String>,
    pub last_login_time: Option<i32>,
}

impl User {
    // add code here
    #[inline]
    pub fn new(uid: i32, mobile: String, password: String) -> Self {
        User{
            uid, 
            mobile, 
            password, 
            nick_name: None, 
            avatar_url: None,
            last_login_time: None
            }
    }

    pub fn set_nick_name(&mut self, nick_name: Option<String>) -> &mut Self {
        self.nick_name = nick_name;
        self
    }

    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) -> &mut Self {
        self.avatar_url = avatar_url;
        self
    }

    pub fn set_last_login_time(&mut self, last_login_time: Option<i32>) -> &mut Self {
        self.last_login_time = last_login_time;
        self
    }
}