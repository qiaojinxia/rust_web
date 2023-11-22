mod routes;
mod config;
mod app;
mod utils; 
use actix_web::{App, HttpServer};
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
            .configure(routes::test::api_config)
            // ... 其他配置 ...
    })
    .bind(format!("{}:{}",globals::APP_CONFIG.server.host, globals::APP_CONFIG.server.port))?
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
