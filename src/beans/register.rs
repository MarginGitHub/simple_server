
#[derive(Debug, FromForm)]
pub struct Register {
    pub mobile: String,
    pub password: String,
}
