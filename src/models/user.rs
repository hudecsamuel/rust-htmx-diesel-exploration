use uuid::Uuid;

// use crate::infra::errors::InfraError;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
