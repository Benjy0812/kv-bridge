use std::env;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Create vault clients
    let cloud_address = env::var("CLOUD_VAULT_ADDR")?;
    let cloud_token = env::var("CLOUD_VAULT_TOKEN")?;
    let antenna_address = env::var("ANTENNA_VAULT_ADDR")?;
    let antenna_token = env::var("ANTENNA_VAULT_TOKEN")?;
    let cloud = make_client(&cloud_address, &cloud_token);
    let antenna = make_client(&antenna_address, &antenna_token);

    // Setup values in CLOUD
    kv2::set(&cloud, "secret", "rotation/secret", &MySecret { secret: "secret-v2".into() }).await?;
    kv2::set(&cloud, "secret", "rotation/antenna_address", &MySecret { secret: "http://127.0.0.1:8210".into() }).await?;

    // send old value to ANTENNA
    kv2::set(&antenna, "secret", "test", &MySecret { secret: "secret-v1".into() }).await?;

    // Before rotation
    println!("Before rotation:");
    let antenna_old_secret: MySecret = kv2::read(&antenna, "secret", "test").await?;
    let cloud_new_secret: MySecret = kv2::read(&cloud, "secret", "rotation/secret").await?;
    println!("Old from antenna: {:?}\nNew from cloud: {:?}", antenna_old_secret, cloud_new_secret);

    // After rotation
    println!("\nAfter rotation:");
    kv2::set(&antenna, "secret", "access_token", &MySecret { secret: "secret-v2".into() }).await?;
    let antenna_new_secret: MySecret = kv2::read(&antenna, "secret", "access_token").await?;

    println!("New from antenna: {:?}\nfrom cloud: {:?}", antenna_new_secret, cloud_new_secret);

    Ok(())
}
