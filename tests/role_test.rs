#[cfg(test)]
mod tests {
    use actix_web::{test, App, http};
    use actix_web::web::Data;
    use my_gpt::{app, routes};
    use my_gpt::config::globals;


    #[actix_rt::test]
    async fn test_delete_role_endpoint() {
        app::init().await;
        let app = test::init_service(
            App::new()
                .app_data(Data::new( globals::APP_STATE.get().clone()
                .expect("DB_POOL not initialized")))
                .configure(routes::admin::role_routes::api_config)
        ).await;

        // 创建一个模拟的DELETE请求
        let req = test::TestRequest::delete()
            .uri("/api/roles/1") // 假设我们想删除ID为1的角色
            .to_request();

        // 发送请求
        let resp = test::call_service(&app, req).await;

        // 验证响应状态码
        assert_eq!(resp.status(), http::StatusCode::OK); // 确认我们收到了200 OK响应
    }
}
