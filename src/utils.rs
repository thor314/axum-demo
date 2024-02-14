use anyhow::{anyhow, Result};
use tracing::trace;
use tracing_subscriber::{
  filter::{EnvFilter, LevelFilter},
  layer::SubscriberExt,
  util::SubscriberInitExt,
};

use crate::error::MyError;
/// Set up crate logging and environment variables.
// #[tracing::instrument]
pub(crate) fn setup() -> Result<(), MyError> {
  dotenv::dotenv().ok();
  let filter =
    EnvFilter::builder().with_default_directive(LevelFilter::INFO.into()).from_env_lossy();
  tracing_subscriber::fmt().with_env_filter(filter).init();

  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  Ok(())
}
