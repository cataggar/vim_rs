use anyhow::Result;
use vim_rs::core::client::ClientBuilder;
use vim_rs::mo::SessionManager;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Get environment variables
    let vc_server = std::env::var("VIM_SERVER").expect("VIM_SERVER env var not set");
    let username = std::env::var("VIM_USERNAME").expect("VIM_USERNAME env var not set");
    let password = std::env::var("VIM_PASSWORD").expect("VIM_PASSWORD env var not set");
    let insecure = std::env::var("VIM_INSECURE").map(|v| v != "false").unwrap_or(false);

    // Create client
    let client = ClientBuilder::new(&vc_server)
        .insecure(insecure)
        .basic_authn(&username, &password)
        .app_details("session_example", "0.1.0")
        .build()
        .await?;

    println!("Connected to: {}", client.service_content().about.full_name);

    // Get the session manager from service content
    let session_manager_ref = &client.service_content().session_manager;
    let session_manager = SessionManager::new(
        client.clone(), 
        &session_manager_ref.as_ref().unwrap().value
    );

    // Get current session directly through SessionManager API
    let current_session = session_manager.current_session().await?;
    
    if let Some(session) = current_session {
        println!("Current session user: {}", session.user_name);
        println!("Session key: {}", session.key);
        println!("Login time: {}", session.login_time);
        println!("Last active time: {}", session.last_active_time);
    } else {
        println!("No current session found");
    }

    Ok(())
}