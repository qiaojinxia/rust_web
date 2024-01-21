mod routes;
mod config;
mod app;
mod common;
mod schema;
mod dto;
mod services;
use actix_web::{web::Data, App, HttpServer};
use std::sync::mpsc;
use std::thread;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
use tokio::sync::oneshot;
use config::globals;


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
            .app_data(Data::new( globals::DB_POOL.get()
            .expect("DB_POOL not initialized").clone())) // 存储应用状态
            .configure(routes::admin::auth_routes::api_config)
            
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
