use rocket::http::Status;
use rocket::response::{status, Redirect};
use rocket::State;
use super::structs;

#[get("/<shorten_to>")]
pub async fn redirect(shorten_to: String, state: &State<structs::MyState>) -> Result<Redirect, status::Custom<String>> {
    let urls: structs::Urls = sqlx::query_as("SELECT * FROM urls WHERE shorten_to = $1")
        .bind(&shorten_to)
        .fetch_one(&state.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => status::Custom(
                Status::NotFound,
                "Url does not exist.".into()
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Something went wrong: internal server error".into()
            )
        })?;

    Ok(Redirect::to(urls.url))
}