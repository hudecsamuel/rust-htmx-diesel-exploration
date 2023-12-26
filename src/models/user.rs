use uuid::Uuid;

// use crate::infra::errors::InfraError;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

// #[derive(Debug)]
// pub enum PostError {
//     InternalServerError,
//     NotFound(Uuid),
//     InfraError(InfraError),
// }

// impl IntoResponse for PostError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, err_msg) = match self {
//             Self::NotFound(id) => (
//                 StatusCode::NOT_FOUND,
//                 format!("PostModel with id {} has not been found", id),
//             ),
//             Self::InfraError(db_error) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Internal server error: {}", db_error),
//             ),
//             _ => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 String::from("Internal server error"),
//             ),
//         };
//         (
//             status,
//             Json(
//                 json!({"resource":"PostModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
//             ),
//         )
//             .into_response()
//     }
// }