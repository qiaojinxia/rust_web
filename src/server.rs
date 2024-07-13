use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};
use actix_web::{web, web::Data, App, HttpServer};
use actix_web::middleware::Logger;
use time::Duration;
use tokio::sync::oneshot;
use crate::config::globals;
use crate::config::globals::AppState;
use crate::{handlers, middleware};

pub async fn start_server(shutdown_rx: oneshot::Receiver<()>) -> std::io::Result<()> {
    let redis_config = &globals::APP_CONFIG.redis;
    let redis_url = format!(
        "redis://:{}@{}:{}",
        redis_config.password.clone().unwrap_or("".to_string()),
        redis_config.host,
        redis_config.port
    );

    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();

    let app_state = globals::APP_STATE.get().unwrap();
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), Key::generate())
                    .cookie_http_only(false)
                    .cookie_same_site(SameSite::None)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl_extension_policy(
                                actix_session::config::TtlExtensionPolicy::OnStateChanges,
                            )
                            .session_ttl(Duration::days(7)),
                    )
                    .build(),
            )
            .app_data(Data::new(AppState {
                redis_conn: app_state.redis_conn.clone(),
                mysql_conn: app_state.mysql_conn.clone(),
            }))
            .service(
                web::scope("/auth").configure(handlers::admin::sys_auth_handler::api_config),
            )
            .service(
                web::scope("/system-manage")
                    .configure(handlers::admin::sys_role_handler::api_config)
                    .configure(handlers::admin::sys_menu_handler::api_config)
                    .configure(handlers::admin::sys_permission_handler::api_config)
                    .configure(handlers::admin::sys_user_role_handler::api_config)
                    .configure(handlers::admin::sys_role_permission_handler::api_config)
                    .configure(handlers::admin::sys_user_handler::api_config)
                    .wrap(middleware::jwt_auth_middleware::JWTAuth),
            )
            .service(
                web::scope("/route")
                    .configure(handlers::admin::sys_route_handler::api_config)
            )
            .wrap(Logger::new("%a %D ms %{User-Agent}i"))
    })
        .bind(format!(
            "{}:{}",
            globals::APP_CONFIG.server.host,
            globals::APP_CONFIG.server.port
        ))?
        .run();

    tokio::select! {
        _ = server => {},
        _ = shutdown_rx => {},
    }

    Ok(())
}
