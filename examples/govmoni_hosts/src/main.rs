//! Govmoni-Compatible Host Summary Example
//!
//! This example mirrors the govmomi Go example in `examples/hosts/main.go` by:
//! - Connecting to vCenter (env controlled)
//! - Retrieving HostSystem summary properties in a single batched call
//! - Computing total/free CPU (MHz) and total/free Memory (bytes)
//! - Printing a tabular output similar to govmomi
//!
//! Environment variables (parallel to govmomi):
//!   GOVMOMI_URL       - https://host/sdk (or blank to error)
//!   GOVMOMI_USERNAME  - if not embedded in URL
//!   GOVMOMI_PASSWORD  - if not embedded in URL
//!   GOVMOMI_INSECURE  - "1"/"true" to skip TLS verification
//!
//! Example:
//!   export GOVMOMI_URL=https://vc.example.com
//!   export GOVMOMI_USERNAME=administrator@vsphere.local
//!   export GOVMOMI_PASSWORD='***'
//!   export GOVMOMI_INSECURE=1
//!   cargo run -p govmoni_hosts --release

use anyhow::{anyhow, Result};
use std::env;
use std::fmt::Write as _;
use vim_macros::vim_retrievable;
use vim_rs::core::pc_retrieve::ObjectRetriever;
use utils::connect_with;

vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        summary = "summary",
    }
);

fn parse_bool(v: &str) -> bool { matches!(v.to_ascii_lowercase().as_str(), "1"|"true"|"t"|"y"|"yes") }

fn main() -> Result<()> {
    env_logger::init();
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let url = env::var("GOVMOMI_URL")
            .or_else(|_| env::var("VIM_SERVER"))
            .map_err(|_| anyhow!("Missing GOVMOMI_URL (or VIM_SERVER) env var"))?;
        let username = env::var("GOVMOMI_USERNAME").ok().or_else(|| env::var("VIM_USERNAME").ok());
        let password = env::var("GOVMOMI_PASSWORD").ok().or_else(|| env::var("VIM_PASSWORD").ok());
        let insecure = env::var("GOVMOMI_INSECURE")
            .or_else(|_| env::var("VIM_INSECURE"))
            .ok()
            .map(|s| parse_bool(&s))
            .unwrap_or(true);

        let client = connect_with(
            &url,
            username.as_deref(),
            password.as_deref(),
            insecure,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ).await?;

        let retriever = ObjectRetriever::new(client.clone())?;
        let hosts: Vec<Host> = retriever
            .retrieve_objects_from_container(&client.service_content().root_folder)
            .await?;

        let mut out = String::new();
        writeln!(&mut out, "Name:\tUsed CPU:\tTotal CPU:\tFree CPU:\tUsed Memory:\tTotal Memory:\tFree Memory:\t").unwrap();

        for h in hosts {
            let name = h.name;
            let summary = &h.summary; // Not optional
            let hw = summary.hardware.as_ref();
            let qs = &summary.quick_stats; // Not optional

            let cpu_mhz_per_core = hw.map(|hw| hw.cpu_mhz).unwrap_or_default() as i64;
            let cores = hw.map(|hw| hw.num_cpu_cores).unwrap_or_default() as i64;
            let total_cpu = cpu_mhz_per_core * cores; // MHz
            let used_cpu = qs.overall_cpu_usage.unwrap_or_default() as i64; // MHz
            let free_cpu = total_cpu - used_cpu;

            let total_mem_bytes = hw.map(|hw| hw.memory_size).unwrap_or_default() as i64; // bytes
            let used_mem_mb = qs.overall_memory_usage.unwrap_or_default() as i64; // MB
            let used_mem_bytes = used_mem_mb * 1024 * 1024; // convert MB -> bytes
            let free_mem_bytes = total_mem_bytes - used_mem_bytes;

            writeln!(
                &mut out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
                name,
                used_cpu,
                total_cpu,
                free_cpu,
                used_mem_bytes,
                total_mem_bytes,
                free_mem_bytes
            ).unwrap();
        }

        print!("{}", out);
        Ok(())
    })
}
