use crate::controllers::html_response::HtmlResponse;
use crate::repositories::user_repository;
use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use diesel::connection;

use crate::db::schema::users;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(self::get::login))
        .route("/login", post(self::post::login))
        .route("/register", get(self::get::register))
        .route("/register", post(self::post::register))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

mod get {
    use super::*;

    pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
        let template = LoginTemplate {};

        return HtmlResponse(template);
    }

    pub async fn register(State(state): State<AppState>) -> impl IntoResponse {
        let template = RegisterTemplate {};

        return HtmlResponse(template);
    }
}

mod post {
    use crate::repositories::{
        auth_backend::AuthSession,
        user_repository::{Credentials, NewUserDb},
    };
    use axum::{http::StatusCode, Form};

    use super::*;
    pub async fn login(
        mut auth_session: AuthSession,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        println!("Creds: {:?}", creds);

        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return HtmlResponse(LoginTemplate {}).into_response();
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if let Some(ref next) = creds.next {
            Redirect::to(next).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }

    pub async fn register(
        State(state): State<AppState>,
        mut auth_session: AuthSession,
        Form(creds): Form<NewUserDb>,
    ) -> impl IntoResponse {
        println!("Creds: {:?}", creds);
        let mut connection = state.pool.get().unwrap();

        let user = tokio::task::spawn_blocking(move || {
            user_repository::create_user(&mut connection, creds)
        })
        .await
        .unwrap()
        .unwrap();

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        Redirect::to("/").into_response()
    }
}
