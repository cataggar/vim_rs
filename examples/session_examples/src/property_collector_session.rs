use anyhow::Result;
use std::env;
use vim_rs::core::client::ClientBuilder;
use vim_rs::mo::PropertyCollector;
use vim_rs::types::structs::{PropertyFilterSpec, ObjectSpec, PropertySpec, RetrieveOptions};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Set HTTP_PROXY for testing
    env::set_var("HTTP_PROXY", "http://127.0.0.1:8080");

    // Get environment variables
    let vc_server = env::var("VIM_SERVER").expect("VIM_SERVER env var not set");
    let username = env::var("VIM_USERNAME").expect("VIM_USERNAME env var not set");
    let password = env::var("VIM_PASSWORD").expect("VIM_PASSWORD env var not set");
    let insecure = env::var("VIM_INSECURE").map(|v| v != "false").unwrap_or(false);

    // Create client
    let client = ClientBuilder::new(&vc_server)
        .insecure(insecure)
        .basic_authn(&username, &password)
        .app_details("session_example", "0.1.0")
        .build()
        .await?;

    println!("Connected to: {}", client.service_content().about.full_name);

    // Get the session manager from service content
    let session_manager = &client.service_content().session_manager;
    
    // Get the property collector
    let content = client.service_content();
    let pc = PropertyCollector::new(client.clone(), &content.property_collector.value);
    
    // Create property specification for SessionManager.currentSession
    let prop_spec = PropertySpec {
        r#type: "SessionManager".to_string(),
        path_set: Some(vec!["currentSession".to_string()]),
        all: Some(false),
    };
    
    // Create object specification for the SessionManager
    let obj_spec = ObjectSpec {
        obj: session_manager.as_ref().unwrap().clone(),
        skip: Some(false),
        select_set: None,
    };
    
    // Create the property filter spec
    let spec = PropertyFilterSpec {
        prop_set: vec![prop_spec],
        object_set: vec![obj_spec],
        report_missing_objects_in_results: None,
    };
    
    // Options for the retrieve operation
    let options = RetrieveOptions {
        max_objects: Some(1),
    };
    
    // Retrieve properties - this makes the same kind of SOAP call as in a.req
    let results = pc.retrieve_properties_ex(&[spec], &options).await?;
    
    if let Some(retrieve_result) = results {
        for object in retrieve_result.objects {
            println!("Retrieved object: {:?}", object.obj);
            if let Some(prop_set) = object.prop_set {
                for prop in prop_set {
                    println!("Property: {} = {:?}", prop.name, prop.val);
                    
                    // If this is the currentSession property, we can examine the session data
                    if prop.name == "currentSession" {
                        println!("Current session found: {:?}", prop.val);
                    }
                }
            }
        }
    }

    Ok(())
}