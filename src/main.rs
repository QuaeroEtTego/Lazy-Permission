mod bot;
mod config;
mod event;
mod interaction;
mod util;

use std::{env, error::Error, process::ExitCode};

use tokio::runtime::Builder;
use tracing::info;

use bot::Bot;
use config::Config;
use util::{logger, wait_shutdown, Shutdown};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

type LazyPermissionResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn main() -> ExitCode {
    Builder::new_multi_thread()
        .thread_name("LazyPermission")
        .enable_all()
        .build()
        .map(|runtime| match runtime.block_on(async_main()) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Fatal error: {e}");
                ExitCode::FAILURE
            }
        })
        .unwrap_or_else(|e| {
            eprintln!("Failed to build the runtime: {e}");
            ExitCode::FAILURE
        })
}

async fn async_main() -> LazyPermissionResult<()> {
    let config = Config::load()?;

    logger::init(&config.log);

    info!("LazyPermission v{}", VERSION);

    let bot = Bot::new(&config.discord).await?;
    let bot_shutdown = Shutdown::new();

    bot.set_commands(&[]).await?;

    let bot_run = tokio::spawn(bot.start(bot_shutdown.subscriber()));

    tokio::select! {
        result = bot_run => result??,
        _ = wait_shutdown() => (),
    }

    info!("Shutting down LazyPermission, ending the last tasks...");

    bot_shutdown.shutdown().await;

    info!("Shutdown complete");

    Ok(())
}
