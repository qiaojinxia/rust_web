// 在您的测试模块中定义一个宏
macro_rules! setup_test_app {
    () => {{
        app::init().await;
        let m = APP_STATE.get().unwrap();
        let app_state = Data::new(AppState{
            redis_conn:  m.redis_conn.clone(),
            mysql_conn: m.mysql_conn.clone(),
        });
        test::init_service(
            App::new()
                .app_data(app_state) // 存储应用状态
                .configure(routes::admin::sys_role_routes::api_config)
                // .wrap(middleware::auth_middleware::JWTAuth)
                .wrap(Logger::default())
                .wrap(Logger::new("%a %D ms %{User-Agent}i"))
        ).await
    }};
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App, http};
    use actix_web::middleware::Logger;
    use actix_web::web::{Data};
    use my_gpt::{app, common, routes};
    use my_gpt::config::globals;
    use my_gpt::config::globals::AppState;
    use crate::tests::globals::APP_STATE;
    use my_gpt::dto::admin::sys_role_dto::RoleCreationDto;
    use my_gpt::dto::admin::sys_role_dto::RoleCreationResponseDto;
    #[actix_rt::test]
    async fn test_create_role() {
        let app = setup_test_app!();

        let role_creation_dto = RoleCreationDto {
            role_code: Some("role_id_2".to_string()),
            role_name: Some("Test Role Name2".to_string()),
            description: Some("Test Role Description".to_string()),
            status: 1,
        };

        let req = test::TestRequest::post()
            .uri("/roles")
            .set_json(&role_creation_dto)
            .to_request();

        let resp: common::resp::ApiResponse<RoleCreationResponseDto> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);

        assert_eq!(resp.data.base.role_code, role_creation_dto.role_code);
    }

    use my_gpt::dto::admin::sys_role_dto::RoleResponseDto;
    #[actix_rt::test]
    async fn test_get_roles() {
        let app = setup_test_app!();

        let req = test::TestRequest::get()
            .uri("/roles/12")
            .to_request();

        let resp: common::resp::ApiResponse<RoleResponseDto> = test::call_and_read_body_json(&app, req).await;

        assert!(!resp.data.role.is_none()); // 假设数据库中至少有一个角色
    }

    use my_gpt::dto::admin::sys_role_dto::RoleUpdateDto;
    use my_gpt::dto::admin::sys_role_dto::RoleUpdateRespDto;
    #[actix_rt::test]
    async fn test_update_role() {
        let app = setup_test_app!();

        let role_update_dto = RoleUpdateDto {
            role_code: Some("new_role_code".to_string()),
            role_name: Some("new_role_name".to_string()),
            description: Some("Updated Role Description".to_string()),
            status: Some(0),
        };

        let req = test::TestRequest::put()
            .uri("/roles/13") // 假设ID为1的角色存在
            .set_json(&role_update_dto)
            .to_request();

        let resp :common::resp::ApiResponse<RoleUpdateRespDto>= test::call_and_read_body_json(&app, req).await;

        assert_eq!(resp.data.role.unwrap().role_name, role_update_dto.role_name);
    }


    use my_gpt::dto::admin::sys_role_dto::RoleDeleteRespDto;
    #[actix_rt::test]
    async fn test_delete_role_endpoint() {
        let app = setup_test_app!();

        // 创建一个模拟的DELETE请求
        let req = test::TestRequest::delete() // 注意这里应该使用delete()来模拟DELETE请求
            .uri("/roles/21") // 假设我们想删除ID为1的角色
            .to_request();

        // 发送请求
        let resp:common::resp::ApiResponse<RoleDeleteRespDto> = test::call_and_read_body_json(&app, req).await;

        // 验证响应状态码
        assert_eq!(resp.code, http::StatusCode::OK); // 确认我们收到了200 OK响应
    }

}
