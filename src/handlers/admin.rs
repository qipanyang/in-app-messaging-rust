use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::admin::{create, find, Admin, NewAdmin};
use crate::models::user::find as find_user;
use crate::validate::validate;
use actix_web::web::{block, Data, Json, Path};
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AdminResponse {
    pub id: i32,
    pub user_id: i32,
    pub role: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateAdminRequest {
    #[validate(length(min = 3, message = "id length must be greater than 3"))]
    pub username: String,
    pub user_role: i32,
}

pub async fn get_admin(
    user_id: Path<i32>,
    pool: Data<PoolType>,
) -> Result<Json<AdminResponse>, ApiError> {
    let admin = block(move || find(&pool, user_id.to_owned())).await?;
    respond_json(admin)
}

pub async fn assign_admin(
    pool: Data<PoolType>,
    params: Json<CreateAdminRequest>,
) -> Result<Json<AdminResponse>, ApiError> {
    validate(&params)?;

    let admin = block(move || {
        let user = find_user(&pool, &params.username)?;
        let new_admin: NewAdmin = NewAdmin {
            user_id: user.id,
            user_role: params.user_role,
        };
        create(&pool, &new_admin)
    })
    .await?;

    respond_json(admin)
}

impl From<Admin> for AdminResponse {
    fn from(admin: Admin) -> Self {
        AdminResponse {
            id: admin.id,
            user_id: admin.user_id,
            role: admin.user_role,
        }
    }
}
