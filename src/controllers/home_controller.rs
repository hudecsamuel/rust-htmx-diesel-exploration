use crate::controllers::html_response::HtmlResponse;
use crate::repositories::auth_backend::Backend;
use crate::repositories::user_repository;
use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use axum_login::login_required;
use crate::AppState;


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(self::get::home))
        .route("/todos", get(self::get::todos))
        .route("/feed", get(self::get::feed))
        .route_layer(login_required!(Backend, login_url = "/login"))
}

mod get {
    use super::*;

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
    
        return HtmlResponse(template);
    }
    
    #[derive(Template)]
    #[template(path = "todos.html")]
    pub struct TodosFragmentTemplate {}
    
    pub async fn todos() -> impl IntoResponse {
        let template = TodosFragmentTemplate {};
    
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
        let template = FeedTemplate { name: "Samuel" };
    
        return HtmlResponse(template);
    }
}


