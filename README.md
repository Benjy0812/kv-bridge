# kv-bridge

## Made in Rust 2024 edition
### Dependencies
````toml
dotenvy = "0.15.7"
serde = "1.0.229"
tokio = { version = "1.54.1", features = ["full"] }
vaultrs = "0.8.0"
````

## one origin of truth the cloud and antenna(s) can only get data

### Simple concept showing the logic of how this can be used in a larger project
## The flow
### AWX gets bootstrap token from CLOUD vault and sends it to BRIDGE, ANTENNA vault and ANTENNA docker then BRIDGE requests token and antenna address from cloud and receives token and antenna address the BRIDGE pushes 

### the setup.bash is made by ai 