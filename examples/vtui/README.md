# vTUI: VMware VM visualization for the terminal

vTUI is a tool that allows you to browse the vCenter inventory in the terminal. It is a simple tool
that uses the VMware API to monitor vCenter inventory and render it in a terminal window.

vTUI's main purpose is to demonstrate how to use the vim_rs library to interact with the VMware API
in a Text User Interface (TUI) application.

vTUI uses the `PropertyCollector` API to retrieve inventory object from the vCenter server. It then
displays the VMs in a terminal window using the Ratatui library.

## Features

- Visualize vCenter inventory directly in your terminal.
- Real-time inventory updates using the PropertyCollector API.
- Search for specific objects using "/"
- Navigate to related objects using shortcuts (v)m, (n)etwork, (h)ost, (d)atastore
- Dive into the details of a VM, Host etc.
- Save an object details to a file
- Go back to the previous view using Backspace
- Browse the Cluster, Host, VM, Network and Datastore inventory
- TUI built with the Ratatui library for a smooth user experience.
- Clean and minimalistic design suitable for server environments.
- Logging support for debugging and monitoring.

## Installation

Ensure you have Rust 1.85 installed. Then set the following environment variables:

- `VIM_SERVER` - Server address of a vCenter server or simulator (version 8.0.2 or later). Can include protocol (`http://` or `https://`). If no protocol is specified, defaults to HTTPS.
- `VIM_USERNAME` - Username for vCenter authentication.
- `VIM_PASSWORD` - Password for vCenter authentication.
- `VIM_INSECURE` - Set to `true` to ignore SSL certificate validation for HTTPS connections (not recommended for production).
- `LOG_LEVEL` - Set to `debug` or `trace` for verbose logging (optional).

## Usage

Set the required environment variables and run vTUI:

**PowerShell (with vcsim):**

```powershell
$env:VIM_SERVER = "http://localhost:8989"
$env:VIM_USERNAME = "user"
$env:VIM_PASSWORD = "pass"
cargo run --bin vtui
```

**Nu Shell (with vcsim):**

```nu
$env.VIM_SERVER = "http://localhost:8989"
$env.VIM_USERNAME = "user"
$env.VIM_PASSWORD = "pass"
$env.LOG_LEVEL = "trace"
$env.HTTP_PROXY = "http://127.0.0.1:8080" # Optional proxy to see usage
cargo run --bin vtui
```

**For real vCenter (HTTPS):**

```powershell
$env:VIM_SERVER = "vcenter.example.com"
$env:VIM_USERNAME = "administrator@vsphere.local"
$env:VIM_PASSWORD = "your-password"
$env:VIM_INSECURE = "true"  # Only if using self-signed certificates
cargo run --bin vtui
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your
improvements.
