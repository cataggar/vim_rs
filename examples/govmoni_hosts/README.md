# Govmoni-Compatible Host Summary Example

This example mirrors the logic of the Go `govmomi` example at `examples/hosts/main.go` by
retrieving host summary statistics (CPU + Memory) and printing a tabular report.

## Features
- Single batched retrieval of HostSystem `summary` property via `vim_retrievable!`
- Computes Used/Total/Free CPU (MHz) and Memory (bytes)
- Environment variable contract aligned with govmomi (`GOVMOMI_*`)

## Env Vars
Set the following before running (same naming as govmomi):
```
export GOVMOMI_URL=https://vcenter.example.com
export GOVMOMI_USERNAME=administrator@vsphere.local
export GOVMOMI_PASSWORD='your_password'
export GOVMOMI_INSECURE=1   # optional; skips TLS verify
```

## Run
From the repo root:
```
cargo run -p govmoni_hosts --release
```

Sample output (columns separated by `\t`):
```
Name:\tUsed CPU:\tTotal CPU:\tFree CPU:\tUsed Memory:\tTotal Memory:\tFree Memory:\t
esx01.lab\t934\t56000\t55066\t17179869184\t34359738368\t17179869184\t
```

## Differences vs govmomi Go example
- Uses Rust macro-generated typed struct instead of manual `view` + `property` usage.
- Memory units: used memory is converted from MB (as reported) to bytes to match total memory units.

## Implementation Notes
The macro definition:
```rust
vim_retrievable!(
    struct Host: HostSystem {
        name = "name",
        summary = "summary",
    }
);
```
`summary` gives access to nested `hardware` and `quick_stats` required for calculations.

## Next Ideas
- Add JSON output flag
- Optional pretty table formatting
- Extend to VM or Datastore summaries similar to govmomi examples
