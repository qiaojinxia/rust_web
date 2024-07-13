
use rust_web::app;
use rust_web::signals;
use rust_web::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::initialize().await;

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

    signals::setup_signals(shutdown_tx);

    server::start_server(shutdown_rx).await
}
