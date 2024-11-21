mod modules;

#[macro_use]
extern crate rocket;

use rocket::routes;
use sqlx::{Executor, PgPool};
use modules::{structs, redirect, shorten};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    pool.execute(include_str!("../migrations/schema.sql"))
        .await
        .expect("Failed to run migrations");

    let state = structs::MyState { pool };
    let rocket = rocket::build().mount("/", routes![redirect::redirect, shorten::shorten]).manage(state);

    Ok(rocket.into())
}