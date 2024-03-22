use actix_web::{web::Data, App, HttpServer, web};
use std::sync::mpsc;
use std::thread;
use actix_cors::Cors;
use actix_web::cookie::{Key, SameSite};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_session::config::PersistentSession;

use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use tokio::sync::oneshot;
use my_gpt::{app, middleware, handlers};
use my_gpt::config::globals;
use actix_web::middleware::Logger;
use time::Duration;
use my_gpt::config::globals::AppState;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    app::init().await;
    // 信号处理
    let (tx, rx) = mpsc::sync_channel(1);
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            tx.send(sig).unwrap()}
    });

    let redis_config = &globals::APP_CONFIG.redis;
    let redis_url = format!("redis://:{}@{}:{}",
                            redis_config.password.clone().unwrap_or("".to_string()),
                            redis_config.host,
                            redis_config.port);

    let redis_store = RedisSessionStore::new(redis_url).await
        .unwrap();

    let app_state = globals::APP_STATE.get().unwrap();
    // 创建 HTTP 服务器
    let server = HttpServer::new( move || {

        let cors = Cors::default()
            .allow_any_origin() // 注意：在生产环境中，你可能想要更严格地配置这个
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(SessionMiddleware::builder(redis_store.clone(), Key::generate())
                      // allow the cookie to be accessed from javascript
                      .cookie_http_only(false)
                      // allow the cookie only from the current domain
                      .cookie_same_site(SameSite::None)
                      .session_lifecycle(
                          PersistentSession::default()
                              .session_ttl_extension_policy(actix_session::config::TtlExtensionPolicy::OnStateChanges)
                              .session_ttl(Duration::days(7)),
            )
                      .build(),
            )
            .app_data(Data::new(AppState{
                redis_conn:  app_state.redis_conn.clone(),
                mysql_conn: app_state.mysql_conn.clone(),
            })).service(
            web::scope("/auth").configure(handlers::admin::sys_auth_routes::api_config) // auth相关配置
            ).service(
                web::scope("/api")
                    .configure(handlers::admin::sys_role_routes::api_config) // role相关配置
                    .configure(handlers::admin::sys_menu_routes::api_config) // role相关配置
                    .configure(handlers::admin::sys_permission_routes::api_config) // role相关配置
                    .configure(handlers::admin::sys_user_role_routes::api_config) // role_user相关配置
                    .configure(handlers::admin::sys_role_permission_routes::api_config) // role_user相关配置
                    .wrap(middleware::permission_check_middleware::PermissionCheck)
                    .wrap(middleware::jwt_auth_middleware::JWTAuth)
            ) // 应用CORS中间件
            .wrap(Logger::new("%a %D ms %{User-Agent}i"))
    }).bind(format!("{}:{}",
        globals::APP_CONFIG.server.host,
        globals::APP_CONFIG.server.port))?
        .run();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    thread::spawn(move || {
        let _ = rx.recv();
        let _ = shutdown_tx.send(());
    });

    tokio::select! {
        _ = server => {},
        _ = shutdown_rx => {},
    }

    Ok(())
}
