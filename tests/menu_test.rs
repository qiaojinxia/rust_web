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
                .configure(routes::admin::sys_menu_routes::api_config)
                // .wrap(middleware::auth_middleware::JWTAuth)
                .wrap(Logger::default())
                .wrap(Logger::new("%a %D ms %{User-Agent}i")),
        )
        .await
    }};
}

#[cfg(test)]
mod menu_tests {
    use actix_web::middleware::Logger;
    use actix_web::web::Data;
    use actix_web::{http, test, App};
    use rust_web::config::globals::AppState;
    use rust_web::config::globals::APP_STATE;
    use rust_web::{app, common, handlers};

    use rust_web::dto::admin::sys_menu_dto::{
        MenuBaseDto, MenuCreateDto, MenuCreationResponseDto, MenuDeleteResponseDto, MenuDto,
        MenuUpdateDto, MenuUpdateResponseDto, MenusResponseDto,
    };

    // Setup test app using the macro

    #[actix_rt::test]
    async fn test_create_menu() {
        let app = setup_test_app!();

        let menu_base_dto = MenuBaseDto {
            menu_name: Some("测试菜单".to_string()),
            icon_type: 1,
            icon: Some("icon_example".to_string()),
            route_name: Some("test_route_name".to_string()),
            route_path: Some("/test/route".to_string()),
            menu_type: 1,
            status: 1,
            is_hidden: 0,
            order: 1,
            ..Default::default()
        };

        // 使用MenuBaseDto实例构造MenuCreationDto
        let menu_creation_dto = MenuCreateDto {
            base: menu_base_dto,
        };

        let req = test::TestRequest::post()
            .uri("/menus")
            .set_json(&menu_creation_dto)
            .to_request();

        let resp: common::resp::ApiResponse<MenuCreationResponseDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_menus() {
        let app = setup_test_app!();
        let req = test::TestRequest::get().uri("/menus").to_request();

        let resp: common::resp::ApiResponse<MenusResponseDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_menu_by_id() {
        let app = setup_test_app!();
        let req = test::TestRequest::get()
            .uri("/menus/1") // Use an appropriate ID
            .to_request();

        let resp: common::resp::ApiResponse<MenuDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
        // Assert on the response content as needed
    }

    #[actix_rt::test]
    async fn test_update_menu() {
        let menu_base_dto = MenuBaseDto {
            menu_name: Some("测试菜单改".to_string()),
            icon_type: 1,
            icon: Some("icon_example".to_string()),
            route_name: Some("test_route_name_update".to_string()),
            route_path: Some("/test/route".to_string()),
            menu_type: 1,
            status: 1,
            is_hidden: 0,
            order: 1,
            ..Default::default()
        };

        let app = setup_test_app!();
        let menu_update_dto = MenuUpdateDto {
            base: menu_base_dto,
        };

        let req = test::TestRequest::put()
            .uri("/menus/1") // Use an appropriate ID
            .set_json(&menu_update_dto)
            .to_request();

        let resp: common::resp::ApiResponse<MenuUpdateResponseDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
        // Assert on the response content as needed
    }

    #[actix_rt::test]
    async fn test_delete_menu() {
        let app = setup_test_app!();
        let req = test::TestRequest::delete()
            .uri("/menus/5") // Use an appropriate ID
            .to_request();

        let resp: common::resp::ApiResponse<MenuDeleteResponseDto> =
            test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
        // Assert on the response content as needed
    }
}
