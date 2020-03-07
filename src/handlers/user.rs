use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::user::{create, find, get_all, NewUser, User};
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use rayon::prelude::*;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UsersResponse(pub Vec<UserResponse>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "id length must be greater than 3"))]
    pub username: String,
}

/// Get a user
pub async fn get_user(
    username: Path<String>,
    pool: Data<PoolType>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = block(move || find(&pool, &username)).await?;
    respond_json(user)
}

/// Get all users
pub async fn get_users(pool: Data<PoolType>) -> Result<Json<UsersResponse>, ApiError> {
    let users = block(move || get_all(&pool)).await?;
    respond_json(users)
}

/// Create a user
pub async fn create_user(
    pool: Data<PoolType>,
    params: Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    let new_user: NewUser = NewUser {
        username: params.username.to_owned(),
    };
    let user = block(move || create(&pool, &new_user)).await?;
    respond_json(user)
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
        }
    }
}

impl From<Vec<User>> for UsersResponse {
    fn from(users: Vec<User>) -> Self {
        UsersResponse(users.into_par_iter().map(|user| user.into()).collect())
    }
}
