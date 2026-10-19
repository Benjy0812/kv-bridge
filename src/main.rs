use serde::{Deserialize, Serialize};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;

#[derive(Debug, Serialize, Deserialize)]
struct MySecret {
    secret: String,
}

fn make_client(addr: &str, token: &str) -> VaultClient {
    VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(addr)
            .token(token)
            .build()
            .unwrap(),
    ).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // create vault objects
    let cloud = make_client("http://127.0.0.1:8200", "cloud");
    let antenna = make_client("http://127.0.0.1:8210", "antenna");

    // set CLOUD to secret/test: "from cloud"
    kv2::set(&cloud, "secret", "test", &MySecret { secret: "from cloud".into() }).await?;

    // get CLOUD data
    let c: MySecret = kv2::read(&cloud, "secret", "test").await?;

    // set ANTENNA to secret/test: data from CLOUD
    kv2::set(&antenna, "secret", "test", &c).await?;

    // get ANTENNA data
    let a: MySecret = kv2::read(&antenna, "secret", "test").await?;

    // get CLOUD and ANTENNA data and print
    println!("Cloud: {:?}\nAntenna: {:?}", c, a);

    Ok(())
}