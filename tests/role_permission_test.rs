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
                .configure(routes::admin::sys_role_permission_routes::api_config)
                // .wrap(middleware::auth_middleware::JWTAuth)
                .wrap(Logger::default())
                .wrap(Logger::new("%a %D ms %{User-Agent}i"))
        ).await
    }};
}

#[cfg(test)]
mod role_permission_tests {
    use actix_web::{test, http};
    use actix_web::web::Data;
    use my_gpt::{app, common, routes};
    use my_gpt::config::globals::AppState;
    use my_gpt::dto::admin::sys_role_permission_dto::{AssignPermissionsDto, RolePermissionsRespDto,
                                                      RemovePermissionRespDto, AssignPermissionsRespDto};
    use my_gpt::config::globals::APP_STATE;
    use actix_web::App;
    use actix_web::middleware::Logger;
    use my_gpt::services::admin::sys_role_permission_services::get_menus_by_role_id;

    // 测试为指定角色分配权限
    #[actix_rt::test]
    async fn test_assign_permissions_to_role() {
        let app = setup_test_app!();

        let assign_permissions_dto = AssignPermissionsDto {
            permission_ids: vec![1, 2], // 假设权限ID为1和2
        };

        let req = test::TestRequest::post()
            .uri("/roles/1/permissions") // 假设角色ID为1
            .set_json(&assign_permissions_dto)
            .to_request();

        let resp: common::resp::ApiResponse<AssignPermissionsRespDto> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
    }

    // 测试获取指定角色的所有权限
    #[actix_rt::test]
    async fn test_get_permissions_of_role() {
        let app = setup_test_app!();

        let req = test::TestRequest::get()
            .uri("/roles/1/permissions") // 假设角色ID为1
            .to_request();

        let resp: common::resp::ApiResponse<RolePermissionsRespDto> = test::call_and_read_body_json(&app, req).await;
        assert!(!resp.data.permissions.is_empty()); // 假设角色至少有一个权限
    }

    // 测试删除指定角色的指定权限
    #[actix_rt::test]
    async fn test_remove_permission_from_role() {
        let app = setup_test_app!();

        let req = test::TestRequest::delete()
            .uri("/roles/1/permissions/1") // 假设角色ID为1，权限ID为1
            .to_request();

        let resp: common::resp::ApiResponse<RemovePermissionRespDto> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK); // 确认我们收到了200 OK响应
    }

    //测试 获取角色所有菜单
    #[actix_rt::test]
    async fn test_get_menus_by_role_id(){
        app::init().await;
        let app_state = APP_STATE.get().unwrap();
        let result = get_menus_by_role_id(&app_state.mysql_conn.clone(),vec![1,2]).await;
        // 检查结果是否成功
        assert!(result.is_ok());
        let menus = result.unwrap();
        // 检查是否至少有一个菜单返回
        // 注意：这个断言可能需要根据你的具体测试数据库内容进行调整
        assert!(!menus.is_empty());
    }
}
