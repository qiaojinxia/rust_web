use actix_web::{web::Data, App, HttpServer};
use std::sync::mpsc;
use std::thread;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use tokio::sync::oneshot;
use my_gpt::{app, routes, middleware};
use my_gpt::config::globals;
use actix_web::middleware::Logger;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::init().await;
    // 信号处理
    let (tx, rx) = mpsc::sync_channel(1);
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            tx.send(sig).unwrap();
        }
    });

    // 创建 HTTP 服务器
    let server = HttpServer::new(|| {
        App::new()
            .app_data(Data::new( globals::APP_STATE.get().clone()
            .expect("DB_POOL not initialized"))) // 存储应用状态
            .configure(routes::admin::auth_routes::api_config)
            .wrap(middleware::auth_middleware::JWTAuth)
            .wrap(Logger::default())
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
