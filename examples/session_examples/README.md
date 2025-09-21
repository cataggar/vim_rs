# Session Examples

This crate contains example programs that demonstrate how to make SOAP requests equivalent to captured HTTP requests using the vim_rs library.

## Examples

### property_collector_session

This example demonstrates how to use the PropertyCollector API to retrieve the current session information from the SessionManager. This is equivalent to making a SOAP RetrievePropertiesEx request directly.

**What it does:**

- Connects to vCenter using basic authentication
- Uses PropertyCollector to retrieve the `currentSession` property from SessionManager
- This replicates the SOAP request pattern shown in captured HTTP requests

### session_manager

This example shows how to directly access the SessionManager API to get current session information.

**What it does:**

- Connects to vCenter using basic authentication  
- Creates a SessionManager object from the service content
- Calls the `current_session()` method directly

## Environment Variables

Both examples require these environment variables:

- `VIM_SERVER`: The vCenter server URL (e.g., `https://vcenter.example.com` or `http://localhost:8989` for vcsim)
- `VIM_USERNAME`: Username for authentication
- `VIM_PASSWORD`: Password for authentication  
- `VIM_INSECURE`: Set to `true` to skip TLS certificate verification (optional, defaults to `false`)

## Running the Examples

```bash
# Set environment variables
export VIM_SERVER="http://localhost:8989"
export VIM_USERNAME="user"
export VIM_PASSWORD="password"
export VIM_INSECURE="true"

# Run the PropertyCollector example
cargo run --bin property_collector_session

# Run the SessionManager example
cargo run --bin session_manager
```

## Testing with vcsim

These examples work well with [vcsim](https://github.com/vmware/govmomi/tree/main/vcsim), a vCenter simulator:

```bash
# Install and run vcsim
go install github.com/vmware/govmomi/vcsim@latest
vcsim -httptest.serve localhost:8989

# Then run the examples with the environment variables shown above
```

## SOAP Request Equivalence

The `property_collector_session` example creates a SOAP request equivalent to:

```xml
<soapenv:Body>
    <RetrievePropertiesEx>
        <_this type="PropertyCollector">propertyCollector</_this>
        <specSet>
            <propSet>
                <type>SessionManager</type>
                <pathSet>currentSession</pathSet>
            </propSet>
            <objectSet>
                <obj type="SessionManager">SessionManager</obj>
            </objectSet>
        </specSet>
        <options>
            <maxObjects>1</maxObjects>
        </options>
    </RetrievePropertiesEx>
</soapenv:Body>
```

This demonstrates how vim_rs abstracts the SOAP layer while providing equivalent functionality.