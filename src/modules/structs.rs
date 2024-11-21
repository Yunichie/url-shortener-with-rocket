use rocket::serde::Serialize;
use sqlx::{FromRow, PgPool};

pub struct MyState {
    pub pool: PgPool
}

#[derive(FromForm)]
pub struct ShortenTo {
    pub url: String,
    pub shorten_to: Option<String>
}

#[derive(Serialize, FromRow)]
pub struct Urls {
    pub id: String,
    pub url: String,
    pub shorten_to: String
}