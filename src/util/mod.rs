mod logger;
mod shutdown;

use super::config::LogConfig;

pub use logger::Logger;
pub use shutdown::{wait_shutdown, Shutdown, ShutdownSubscriber};
