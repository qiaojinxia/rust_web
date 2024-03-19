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
                .configure(routes::admin::sys_user_role_routes::api_config)
                // .wrap(middleware::auth_middleware::JWTAuth)
                .wrap(Logger::default())
                .wrap(Logger::new("%a %D ms %{User-Agent}i"))
        ).await
    }};
}
#[cfg(test)]
mod user_role_tests {
    use actix_web::{test, http};
    use actix_web::web::Data;
    use my_gpt::{app, common, routes};
    use my_gpt::config::globals::AppState;
    use my_gpt::dto::admin::sys_user_role_dto::{AssignRolesDto, UserRolesRespDto, RemoveRoleRespDto, AssignRolesRespDto};
    use my_gpt::config::globals::APP_STATE;
    use actix_web::App;
    use actix_web::middleware::Logger;

    #[actix_rt::test]
    async fn test_assign_roles_to_user() {
        let app = setup_test_app!();

        let user_role_assign_dto = AssignRolesDto {
            role_ids: vec![1, 2],
        };

        let req = test::TestRequest::post()
            .uri("/users/1/roles") // 假设用户ID为1
            .set_json(&user_role_assign_dto)
            .to_request();

        let resp: common::resp::ApiResponse<AssignRolesRespDto> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_roles_of_user() {
        let app = setup_test_app!();

        let req = test::TestRequest::get()
            .uri("/users/1/roles") // 假设用户ID为1
            .to_request();

        let resp: common::resp::ApiResponse<UserRolesRespDto> = test::call_and_read_body_json(&app, req).await;
        assert!(!resp.data.roles.is_empty()); // 假设用户至少有一个角色
    }

    #[actix_rt::test]
    async fn test_delete_role_of_user() {
        let app = setup_test_app!();

        let req = test::TestRequest::delete()
            .uri("/users/1/roles/1") // 假设用户ID为1，角色ID为role_id_1
            .to_request();

        let resp: common::resp::ApiResponse<RemoveRoleRespDto> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.code, http::StatusCode::OK); // 确认我们收到了200 OK响应
    }
    

}
