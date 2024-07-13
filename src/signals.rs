use std::sync::mpsc;
use std::thread;
use tokio::sync::oneshot;

#[cfg(unix)]
use signal_hook::consts::signal::{SIGINT, SIGTERM};
#[cfg(unix)]
use signal_hook::iterator::Signals;

pub fn setup_signals(shutdown_tx: oneshot::Sender<()>) {
    let (tx, rx) = mpsc::sync_channel(1);

    #[cfg(unix)]
    {
        let mut signals = Signals::new(&[SIGINT, SIGTERM]).expect("Unable to create signal iterator");
        thread::spawn(move || {
            for sig in signals.forever() {
                tx.send(sig).unwrap();
            }
        });
    }

    #[cfg(windows)]
    {
        ctrlc::set_handler(move || {
            tx.send(()).unwrap();
        }).expect("Error setting Ctrl-C handler");
    }

    thread::spawn(move || {
        let _ = rx.recv();
        let _ = shutdown_tx.send(());
    });
}
