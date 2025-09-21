use anyhow::{Context, Result};
use log::debug;
use std::env;
use std::sync::Arc;
use vim_rs::core::client::{Client, ClientBuilder};

/// Connect to the vSphere server using the credentials provide via environment variables.
pub async fn connect(app_name: &str, app_version: &str) -> Result<Arc<Client>> {
    let vc_server = env::var("VIM_SERVER").with_context(|| "VIM_SERVER env var not set")?;
    let username = env::var("VIM_USERNAME").with_context(|| "VIM_USERNAME env var not set")?;
    let pwd = env::var("VIM_PASSWORD").with_context(|| "VIM_PASSWORD env var not set")?;

    let skip_hello = env::var("VIM_SKIP_HELLO").ok().map(|v| matches!(v.to_ascii_lowercase().as_str(), "1"|"true"|"yes"|"y"|"t" )).unwrap_or(false);
    let client = ClientBuilder::new(vc_server.as_str())
        .insecure(true)
        .basic_authn(username.as_str(), pwd.as_str())
        .app_details(app_name, app_version)
        .skip_hello(skip_hello)
        .build()
        .await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}

/// More flexible connector used by govmoni compatibility example.
/// Accepts explicit URL and optional credentials; if credentials are None it relies on session propagation (e.g. cookie) which currently is not implemented here.
pub async fn connect_with(
    url: &str,
    username: Option<&str>,
    password: Option<&str>,
    insecure: bool,
    app_name: &str,
    app_version: &str,
) -> Result<Arc<Client>> {
    let skip_hello = env::var("VIM_SKIP_HELLO").ok().map(|v| matches!(v.to_ascii_lowercase().as_str(), "1"|"true"|"yes"|"y"|"t" )).unwrap_or(false);
    let mut builder = ClientBuilder::new(url).insecure(insecure).app_details(app_name, app_version).skip_hello(skip_hello);
    if let (Some(u), Some(p)) = (username, password) {
        builder = builder.basic_authn(u, p);
    }
    let client = builder.build().await?;
    debug!("Connected to {}", client.service_content().about.full_name);
    Ok(client)
}
