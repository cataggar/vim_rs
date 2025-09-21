use crate::types::structs::ServiceContent;
use crate::core::client::Error;
type Result<T> = std::result::Result<T, Error>;

/// Perform a minimal SOAP RetrieveServiceContent call to /sdk when JSON endpoints are unavailable.
pub async fn retrieve_service_content(http: &reqwest::Client, base: &str) -> Result<ServiceContent> {
    // base is like https://host/sdk/vim25/<release>; trim to /sdk for SOAP call
    let sdk_pos = base.find("/sdk/").unwrap_or_else(|| base.len());
    let (prefix, _) = base.split_at(sdk_pos + 4); // include /sdk
    let envelope = r#"<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:vim25=\"urn:vim25\"><soapenv:Body><vim25:RetrieveServiceContent><_this type=\"ServiceInstance\">ServiceInstance</_this></vim25:RetrieveServiceContent></soapenv:Body></soapenv:Envelope>"#;
    let url = format!("{}", prefix.trim_end_matches('/'));
    let res = http.post(&url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .body(envelope)
        .send().await?;
    let text = res.text().await?;
    // Extract XML fields we need minimally via naive parsing (about fullName + service references)
    // Here we fallback to an empty ServiceContent because full SOAP -> JSON mapping is non-trivial.
    // For simulator fallback we only need a few managed object references: rootFolder, sessionManager, propertyCollector.
    fn extract(tag: &str, xml: &str) -> Option<String> {
        let open = format!("<{} ", tag); // capture attribute form
        if let Some(pos) = xml.find(&open) {
            // read until '>' then capture inner text until '<'
            let after = &xml[pos..];
            // pattern like <rootFolder type="Folder">group-d1</rootFolder>
            if let Some(gt) = after.find('>') { 
                let rest = &after[gt+1..];
                if let Some(end) = rest.find('<') { return Some(rest[..end].to_string()); }
            }
        }
        None
    }
    let root_folder = extract("rootFolder", &text).unwrap_or_default();
    let pc = extract("propertyCollector", &text).unwrap_or_else(|| "propertyCollector".to_string());
    let sm = extract("sessionManager", &text).unwrap_or_else(|| "SessionManager".to_string());

    // Construct minimal ServiceContent JSON stub. (We rely on serde to deserialize.)
    #[derive(serde::Serialize)]
    struct MinimalRef { #[serde(rename="_typeName")] _type: &'static str, #[serde(rename="type")] r#type: &'static str, value: String }
    #[derive(serde::Serialize)]
    struct MinimalContent { #[serde(rename="_typeName")] _type: &'static str,
        rootFolder: MinimalRef,
        propertyCollector: MinimalRef,
        sessionManager: MinimalRef,
        about: serde_json::Value,
    }
    let json_content = MinimalContent {
        _type: "ServiceContent",
        rootFolder: MinimalRef { _type: "ManagedObjectReference", r#type: "Folder", value: root_folder },
        propertyCollector: MinimalRef { _type: "ManagedObjectReference", r#type: "PropertyCollector", value: pc },
        sessionManager: MinimalRef { _type: "ManagedObjectReference", r#type: "SessionManager", value: sm },
        about: serde_json::json!({"_typeName":"AboutInfo","fullName":"Simulator (SOAP Fallback)","apiType":"VirtualCenter","apiVersion":"sim"}),
    };
    let ser = serde_json::to_vec(&json_content)?;
    let sc: ServiceContent = serde_json::from_slice(&ser)?;
    Ok(sc)
}