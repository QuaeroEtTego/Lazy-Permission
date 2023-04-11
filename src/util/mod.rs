pub mod logger;
mod shutdown;

use super::config::LogConfig;

pub use shutdown::{wait_shutdown, Shutdown, ShutdownSubscriber};
