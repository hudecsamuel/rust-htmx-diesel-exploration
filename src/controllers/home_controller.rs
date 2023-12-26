use crate::controllers::html_response::HtmlResponse;
use crate::repositories::user_repository;
use askama::Template; // Make sure to add askama to your dependencies in Cargo.toml
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use diesel::connection;

use crate::db::schema::users;
use crate::AppState;

#[derive(Template)] // The derive(Template) macro generates the code needed to render your template.
#[template(path = "home.html")] // This specifies the path to the template file.
struct HomeTemplate<'a> {
    // This struct will hold the variables that you'll use in your template.
    pub name: &'a str,
}

pub async fn home(State(state): State<AppState>) -> impl IntoResponse {
    let template = HomeTemplate { name: "Samuel" };
    let mut connection = state.pool.get().unwrap();

    let results = tokio::task::spawn_blocking(move || user_repository::get_users(&mut connection))
        .await
        .unwrap();

    if let Ok(user) = results {
        println!("User: {:?}", user);
    }

    // let result = tokio::task::spawn_blocking(move || {
    //     user_repository::create_user(
    //         &mut connection,
    //         user_repository::NewUserDb {
    //             name: "Samuel".to_string(),
    //             email: "hudecsamuel@gmail.com".to_string(),
    //             password: "whatever".to_string(),
    //         })
    // }).await.unwrap();
    // println!("Result: {:?}", result);

    return HtmlResponse(template);
}

// template for feed
#[derive(Template)] // The derive(Template) macro generates the code needed to render your template.
#[template(path = "feed.html")] // This specifies the path to the template file.
struct FeedTemplate<'a> {
    // TODO: add proper types from models
    pub name: &'a str,
}

pub async fn feed() -> impl IntoResponse {
    let template = HomeTemplate { name: "Samuel" };

    return HtmlResponse(template);
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/feed", get(feed))
}
