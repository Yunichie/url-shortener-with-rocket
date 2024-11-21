use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use url::Url;
use super::structs;

#[post("/", data = "<url>")]
pub async fn shorten(url: Form<structs::ShortenTo>, state: &State<structs::MyState>) -> Result<String, status::Custom<String>> {
    let id = &nanoid::nanoid!(5);
    let parse_url = Url::parse(&url.url).map_err(|err| {
        status::Custom(
            Status::UnprocessableEntity,
            format!("Bad URL: {}", err)
        )
    })?;
    // TODO: check whether shortener already exists
    let shortener_exists: bool;
    let shorten_to = match &url.shorten_to {
        Some(shorten_to) => shorten_to.clone(),
        None => {
            id.clone()
        }
    };

    sqlx::query("INSERT INTO urls(id, url, shorten_to) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(parse_url.as_str())
        .bind(shorten_to.as_str())
        .execute(&state.pool)
        .await
        .map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                "Something went wrong: could not shorten".into()
            )
        })?;

    Ok(format!("https://shorten-dat-url.shuttleapp.rs/{shorten_to}"))
}