use anyhow::{Context, Result};
use quick_xml::Reader;
use regex::Regex;

// This example intentionally performs only the SOAP RetrieveServiceContent call
// that the simulator (vcsim) supports, without touching the JSON property
// endpoints used by the higher-level vim_rs client. It prints the basic
// ServiceInstance content (about + rootFolder reference ids) obtained from the
// SOAP response. This provides a minimal working example under vcsim.

fn main() -> Result<()> {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {

    let url = std::env::var("VIM_SERVER")
        .or_else(|_| std::env::var("GOVMOMI_URL"))
        .context("Set VIM_SERVER or GOVMOMI_URL (https://host:port)")?;
    let username = std::env::var("VIM_USERNAME")
        .or_else(|_| std::env::var("GOVMOMI_USERNAME"))
        .unwrap_or_else(|_| "user".to_string());
    let password = std::env::var("VIM_PASSWORD")
        .or_else(|_| std::env::var("GOVMOMI_PASSWORD"))
        .unwrap_or_else(|_| "pass".to_string());
    let insecure = std::env::var("VIM_INSECURE")
        .or_else(|_| std::env::var("GOVMOMI_INSECURE"))
        .ok()
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(insecure)
        .build()?;

    // Build the SOAP envelope for RetrieveServiceContent.
    let soap = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:vim25="urn:vim25">
  <soapenv:Body>
    <vim25:RetrieveServiceContent>
      <vim25:_this type="ServiceInstance">ServiceInstance</vim25:_this>
    </vim25:RetrieveServiceContent>
  </soapenv:Body>
</soapenv:Envelope>"#);

    let sdk = if url.ends_with("/sdk") || url.ends_with("/sdk/") {
        url.clone()
    } else if url.ends_with('/') {
        format!("{}sdk", url)
    } else {
        format!("{}/sdk", url)
    };

    println!("SOAP endpoint: {}", sdk);

    // Basic auth like govc / govmomi (simulator accepts it inline).
    let resp = client
        .post(&sdk)
        .basic_auth(username, Some(password))
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(soap)
        .send()
        .await
        .context("Sending SOAP RetrieveServiceContent")?;

    let status = resp.status();
    let text = resp.text().await.context("Reading SOAP body")?;

    if !status.is_success() {
        anyhow::bail!("SOAP call failed: {status} body={}", truncate(&text, 400));
    }

    let parsed = parse_service_content(&text)?;

    println!("ServiceContent.about.version        = {}", parsed.about_version.unwrap_or_default());
    println!("ServiceContent.about.fullName       = {}", parsed.about_full_name.unwrap_or_default());
    println!("ServiceContent.rootFolder           = {}", parsed.root_folder.unwrap_or_default());
    println!("ServiceContent.propertyCollector    = {}", parsed.property_collector.unwrap_or_default());
    println!("ServiceContent.viewManager          = {}", parsed.view_manager.unwrap_or_default());

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}…", &s[..max]) }
}

struct ServiceContentExtract {
    about_version: Option<String>,
    about_full_name: Option<String>,
    root_folder: Option<String>,
    property_collector: Option<String>,
    view_manager: Option<String>,
}

fn parse_service_content(xml: &str) -> Result<ServiceContentExtract> {
    // Fast but not fully robust: we scan for specific element names.
    // For a production implementation, build full SOAP + vim25 XML model.
    let _reader = Reader::from_str(xml); // reserved for future structured parsing

    let mut about_version = None;
    let mut about_full_name = None;
    let mut root_folder = None;
    let mut property_collector = None;
    let mut view_manager = None;

    let re_about_version = Regex::new(r"<version>([^<]+)</version>").unwrap();
    let re_about_full = Regex::new(r"<fullName>([^<]+)</fullName>").unwrap();
    let re_root = Regex::new(r"<rootFolder[^>]*>([^<]+)</rootFolder>").unwrap();
    let re_pc = Regex::new(r"<propertyCollector[^>]*>([^<]+)</propertyCollector>").unwrap();
    let re_vm = Regex::new(r"<viewManager[^>]*>([^<]+)</viewManager>").unwrap();

    if let Some(caps) = re_about_version.captures(xml) { about_version = Some(caps[1].to_string()); }
    if let Some(caps) = re_about_full.captures(xml) { about_full_name = Some(caps[1].to_string()); }
    if let Some(caps) = re_root.captures(xml) { root_folder = Some(caps[1].to_string()); }
    if let Some(caps) = re_pc.captures(xml) { property_collector = Some(caps[1].to_string()); }
    if let Some(caps) = re_vm.captures(xml) { view_manager = Some(caps[1].to_string()); }

    Ok(ServiceContentExtract { about_version, about_full_name, root_folder, property_collector, view_manager })
}
