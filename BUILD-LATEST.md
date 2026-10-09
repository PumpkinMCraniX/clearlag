# PumpkinMC API compatibility

This source has its plugin version synchronized across Cargo.toml and PluginMetadata.

IMPORTANT: The dependency currently targets the historical 0.2.0+26.3-26.51 release.
The latest PumpkinMC API could not be verified from this environment.
To build against the same API as your server, replace the `tag = ...` selector
for pumpkin-plugin-api with the exact Git revision used to build your server:

```toml
pumpkin-plugin-api = { git = "https://github.com/Pumpkin-MC/Pumpkin", package = "pumpkin-plugin-api", rev = "YOUR_SERVER_GIT_COMMIT" }
```

Then on CachyOS:
```bash
rustup target add wasm32-wasip2
cargo update
cargo build --release --target wasm32-wasip2
```

Do not deploy without confirming successful build and testing on the target server.
