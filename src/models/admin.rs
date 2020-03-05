use crate::database::PoolType;
use crate::errors::ApiError;
//use crate::handlers::user::{UserResponse, UsersResponse};
use crate::handlers::admin::AdminResponse;
use crate::schema::admins;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Admin {
    pub id: i32,
    pub user_id: i32,
    pub user_role: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Insertable)]
#[table_name = "admins"]
pub struct NewAdmin {
    pub user_id: i32,
    pub user_role: i32,
}

pub fn find(pool: &PoolType, user_id: i32) -> Result<AdminResponse, ApiError> {
    use crate::schema::admins::dsl::{user_id as user_id_pred, admins};
    let not_found = format!("Admin {:?} not found", user_id);
    let conn = pool.get()?;
    let admin = admins
        .filter(user_id_pred.eq(user_id))
        .first::<Admin>(&conn)
        .map_err(|_| ApiError::NotFound(not_found))?;

    Ok(admin.into())
}

/// Create a new admin
pub fn create(pool: &PoolType, new_admin: &NewAdmin) -> Result<AdminResponse, ApiError> {
    use crate::schema::admins::dsl::admins;
    let conn = pool.get()?;
    let user_id = new_admin.user_id.to_owned();
    diesel::insert_into(admins)
        .values(new_admin)
        .execute(&conn)?;
    find(pool, user_id)
}
