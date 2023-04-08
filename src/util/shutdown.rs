use std::io::Result;

use tokio::sync::{broadcast, mpsc};

#[cfg(unix)]
pub async fn wait_shutdown() -> Result<()> {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigquit = signal(SignalKind::quit())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        _ = sigint.recv() => (),
        _ = sigquit.recv() => (),
        _ = sigterm.recv() => (),
    };

    Ok(())
}

#[cfg(not(unix))]
pub async fn wait_shutdown() -> Result<()> {
    tokio::signal::ctrl_c().await
}

#[derive(Debug)]
pub struct Shutdown {
    notify: broadcast::Sender<()>,
    sender: mpsc::Sender<()>,
    receiver: mpsc::Receiver<()>,
}

impl Shutdown {
    pub fn new() -> Self {
        let (notify, _) = broadcast::channel(1);
        let (sender, receiver) = mpsc::channel(1);

        Self {
            notify,
            sender,
            receiver,
        }
    }

    pub fn subscriber(&self) -> ShutdownSubscriber {
        ShutdownSubscriber {
            shutdown: false,
            notify: self.notify.subscribe(),
            _sender: self.sender.clone(),
        }
    }

    pub async fn shutdown(self) {
        let Shutdown {
            notify,
            sender,
            mut receiver,
        } = self;

        drop(notify);
        drop(sender);

        receiver.recv().await;
    }
}

impl Default for Shutdown {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ShutdownSubscriber {
    shutdown: bool,
    notify: broadcast::Receiver<()>,
    _sender: mpsc::Sender<()>,
}

impl ShutdownSubscriber {
    pub async fn wait_shutdown(&mut self) {
        if self.shutdown {
            return;
        }

        let _ = self.notify.recv().await;
        self.shutdown = true
    }
}
