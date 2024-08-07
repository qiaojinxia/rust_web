macro_rules! setup_test_app {
    () => {{
        app::init().await;
        let m = APP_STATE.get().unwrap();
        let app_state = Data::new(AppState {
            redis_conn: m.redis_conn.clone(),
            mysql_conn: m.mysql_conn.clone(),
        });
        test::init_service(
            App::new()
                .app_data(app_state) // 存储应用状态
                .configure(routes::admin::sys_permission_routes::api_config)
                // .wrap(middleware::auth_middleware::JWTAuth)
                .wrap(Logger::default())
                .wrap(Logger::new("%a %D ms %{User-Agent}i")),
        )
        .await
    }};
}

#[cfg(test)]
mod permission_tests {
    use actix_web::middleware::Logger;
    use actix_web::web::Data;
    use actix_web::App;
    use actix_web::{http, test};
    use rust_web::config::globals::AppState;
    use rust_web::config::globals::APP_STATE;
    use rust_web::dto::admin::sys_permission_dto::{
        PermissionCreationDto, PermissionCreationRespDto, PermissionDeleteRespDto,
        PermissionRespDto, PermissionUpdateDto, PermissionUpdateRespDto,
    };
    use rust_web::{app, common, handlers};

    // 这里假设你已经有了setup_test_app宏，可以直接使用
    #[actix_rt::test]
    async fn test_create_permission() {
        let app = setup_test_app!();

        let permission_creation_dto = PermissionCreationDto {
            permission_code: "Test Permission Name2".to_string(),
            description: Some("Test Permission Description2".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/permissions")
            .set_json(&permission_creation_dto)
            .to_request();

        let resp: common::resp::ApiResponse<PermissionCreationRespDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
        assert_eq!(
            resp.data.base.permission_code,
            permission_creation_dto.permission_code
        );
    }

    #[actix_rt::test]
    async fn test_get_permissions() {
        let app = setup_test_app!();

        let req = test::TestRequest::get().uri("/permissions").to_request();

        let resp: common::resp::ApiResponse<PermissionsRespDto> =
            test::call_and_read_body_json(&app, req).await;
        assert!(!resp.data.base.is_empty()); // 假设数据库中至少有一个权限
    }

    #[actix_rt::test]
    async fn test_get_permission_by_id() {
        let app = setup_test_app!();

        let req = test::TestRequest::get()
            .uri("/permissions/1") // 假设ID为12的权限存在
            .to_request();

        let resp: common::resp::ApiResponse<PermissionRespDto> =
            test::call_and_read_body_json(&app, req).await;
        assert!(resp.data.base.id != 0); // 确认我们能获取到权限ID
    }

    #[actix_rt::test]
    async fn test_update_permission() {
        let app = setup_test_app!();

        let permission_update_dto = PermissionUpdateDto {
            permission_code: Some("new_permission_code".to_string()),
            description: Some("Updated Permission Description".to_string()),
        };

        let req = test::TestRequest::put()
            .uri("/permissions/2") // 假设ID为13的权限存在
            .set_json(&permission_update_dto)
            .to_request();

        let resp: common::resp::ApiResponse<PermissionUpdateRespDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            resp.data.base.permission_code,
            permission_update_dto.permission_code.unwrap()
        );
    }

    #[actix_rt::test]
    async fn test_delete_permission() {
        let app = setup_test_app!();

        let req = test::TestRequest::delete()
            .uri("/permissions/1") // 假设ID为21的权限存在
            .to_request();

        let resp: common::resp::ApiResponse<PermissionDeleteRespDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK); // 确认我们收到了200 OK响应
    }
}
