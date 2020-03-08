use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::user::{UserResponse, UsersResponse};
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
}

/// Get all users
pub fn get_all(pool: &PoolType) -> Result<UsersResponse, ApiError> {
    use crate::schema::users::dsl::users;

    let conn = pool.get()?;
    let all_users = users.load(&conn)?;

    Ok(all_users.into())
}

/// Find a user by the user's username or error out
pub fn find(pool: &PoolType, username: &str) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{username as username_pred, users};

    let conn = pool.get()?;
    let user = users
        .filter(username_pred.eq(username))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(format!("User {:?} not found", username.to_owned())))?;

    Ok(user.into())
}

/// Find a user by the user's id or error out
pub fn find_by_id(pool: &PoolType, user_id: i32) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::{id as id_pred, users};

    let conn = pool.get()?;
    let user = users
        .filter(id_pred.eq(user_id))
        .first::<User>(&conn)
        .map_err(|_| ApiError::NotFound(format!("User {} not found", user_id)))?;

    Ok(user.into())
}

/// Create a new user
pub fn create(pool: &PoolType, new_user: &NewUser) -> Result<UserResponse, ApiError> {
    use crate::schema::users::dsl::users;
    let conn = pool.get()?;
    let username = new_user.username.to_owned();
    diesel::insert_into(users).values(new_user).execute(&conn)?;
    find(pool, &username)
}
