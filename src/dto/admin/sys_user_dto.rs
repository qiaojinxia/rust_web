use serde::{Deserialize, Serialize};
use validator_derive::Validate;
use crate::schemas::admin::sys_user::Model;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCreateDto {
    #[serde(rename = "id")]
    pub id: i32,
    #[validate(length(min = 1))]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[validate(email)]
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userPhone")]
    pub user_phone: String,
    #[serde(rename = "userGender")]
    pub user_gender: String,
    // #[validate(range(min = 1, max = 2))]
    #[serde(rename = "status")]
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
pub struct UserWithRolesDto {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "userPhone")]
    pub user_phone: String,
    #[serde(rename = "userGender")]
    pub user_gender: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createBy")]
    pub create_by: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateBy")]
    pub update_by: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "userRoles")]
    pub user_roles: Option<Vec<i32>>,
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
            create_time: user.create_time.map_or_else(|| "".to_string(), |dt| dt.to_string()),
            update_by: user.update_user.unwrap_or_default(),
            update_time: user.update_time.map_or_else(|| "".to_string(), |dt| dt.to_string()),
            user_roles: roles,
        }
    }
}
