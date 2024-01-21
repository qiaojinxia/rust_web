
#[derive(Deserialize, Validate)]
pub struct CreateMenuItemRequest {
    #[validate(length(min = 1, message = "Menu name must not be empty"))]
    pub menu_name: String,
    pub permission_id: i32,
    #[validate(url(message = "Invalid URL"))]
    pub url: String,
    pub sort: i8,
    #[validate(length(min = 1, message = "Style must not be empty"))]
    pub style: String,
    pub parent_id: Option<i32>,
}


#[derive(Deserialize, Validate)]
pub struct UpdateMenuItemRequest {
    pub id: i32,
    #[validate(length(min = 1))]
    pub menu_name: Option<String>,
    pub permission_id: Option<i32>,
    #[validate(url)]
    pub url: Option<String>,
    pub sort: Option<i8>,
    #[validate(length(min = 1))]
    pub style: Option<String>,
    pub parent_id: Option<i32>,
}
