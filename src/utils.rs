use anyhow::anyhow;
use axum::{http::StatusCode, response::Redirect};
use dotenv::var;
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

pub(crate) async fn mongo() -> anyhow::Result<mongodb::Client> {
  use mongodb::{options::ClientOptions, Client};
  let mongo_uri = format!(
    "mongodb+srv://{}:{}@{}",
    var("DB_USERNAME")?,
    var("DB_PASSWORD")?,
    var("MONGO_CLUSTER")?
  );

  // Parse a connection string into an options struct.
  // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
  let mut client_options = ClientOptions::parse(mongo_uri).await?;

  // Manually set an option.
  client_options.app_name = Some("ZKHN".to_string());

  // Get a handle to the deployment.
  let client = Client::with_options(client_options)?;

  // List the names of the databases in that deployment.
  for db_name in client.list_database_names(None, None).await? {
    println!("{}", db_name);
  }
  // Ok(())
  Ok(client)
}

use axum_extra::{
    TypedHeader,
    headers::authorization::{Authorization, Bearer},
    extract::cookie::{CookieJar, Cookie},
};
async fn create_session(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), StatusCode> {
    if let Some(session_id) = authorize_and_create_session(auth.token()).await {
        Ok((
            // the updated jar must be returned for the changes
            // to be included in the response
            jar.add(Cookie::new("session_id", session_id)),
            Redirect::to("/me"),
        ))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn me(jar: CookieJar) -> Result<(), StatusCode> {
    if let Some(session_id) = jar.get("session_id") {
        // fetch and render user...
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}