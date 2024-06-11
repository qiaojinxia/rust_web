use crate::schemas::admin::sys_user::Model;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::dto::admin::common_dto::{validate_mobile, validate_status, validate_gender};

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateDto {
    #[validate(length(min = 1))]
    pub user_name: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub nick_name: String,
    #[validate(email)]
    pub user_email: String,
    #[validate(custom(function = "validate_mobile"))]
    pub user_phone: String,
    #[validate(length(min = 1), custom(function = "validate_gender"))]
    pub user_gender: String,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String,
    #[serde(rename = "userRoles")]
    pub user_roles: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRespDto {
    #[serde(flatten)]
    pub base: UserWithRolesDto,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithRolesDto {
    pub id: i32,
    pub user_name: String,
    pub nick_name: String,
    pub user_email: String,
    pub user_phone: String,
    pub user_gender: String,
    pub status: String,
    pub create_by: String,
    pub create_time: String,
    pub update_by: String,
    pub update_time: String,
    pub user_roles: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub(crate) user_id: String,
    pub(crate) user_name: String,
    pub(crate) buttons: Vec<String>,
    pub(crate) roles: Vec<String>,
}

impl From<(Model, Option<Vec<i32>>)> for UserWithRolesDto {
    fn from((user, roles): (Model, Option<Vec<i32>>)) -> Self {
        UserWithRolesDto {
            id: user.id,
            user_name: user.user_name,
            nick_name: user.nick_name,
            user_email: user.email,
            user_phone: user.mobile.unwrap_or_default(),
            user_gender: "M".to_string(),
            status: user.status.to_string(),
            create_by: user.create_user,
            create_time: user
                .create_time
                .map_or_else(|| "".to_string(), |dt| dt.to_string()),
            update_by: user.update_user.unwrap_or_default(),
            update_time: user
                .update_time
                .map_or_else(|| "".to_string(), |dt| dt.to_string()),
            user_roles: roles,
        }
    }
}
