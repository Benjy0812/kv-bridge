use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::error::ClientError;
use vaultrs::kv2;

#[derive(Debug, Serialize, Deserialize)]
struct MySecret {
    secret: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct GetData {
    name: String,
    age: String,
    birthday: String,
}

// Build a Vault client from an address and token
fn vault_client(addr: &str, token: &str) -> Result<VaultClient, Box<dyn std::error::Error>> {
    Ok(VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(addr)
            .token(token)
            .build()?,
    )?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let sleep_time: u64 = 3;

    // Connect to Central Vault
    let cloud = vault_client(
        &env::var("CLOUD_VAULT_ADDR")?,
        &env::var("NEW_CLOUD_VAULT_TOKEN")?,
    )?;

    // Set initial state in Central Vault
    kv2::set(
        &cloud,
        "secret",
        "rotation/secret",
        &MySecret {
            secret: "bootstrap-token-v0".into(),
        },
    )
        .await?;

    let mut data = HashMap::new();
    data.insert("name", "Albert");
    data.insert("age", "50");
    data.insert("birthday", "10th of October");
    kv2::set(&cloud, "secret", "data", &data).await?;

    // AWX: relay bootstrap token to Antenna-Vault
    let bootstrap: MySecret = kv2::read(&cloud, "secret", "rotation/secret").await?;

    // Antenna address now comes from .env, not from the cloud
    let antenna_addr = env::var("ANTENNA_VAULT_ADDR")?;
    let antenna = vault_client(&antenna_addr, &env::var("OLD_ANTENNA_VAULT_TOKEN")?)?;

    kv2::set(&antenna, "secret", "rotation/secret", &bootstrap).await?;

    println!("\nWaiting for bootstrap token from cloud");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nBridge: Received bootstrap token");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nAntenna-vault: Received bootstrap token");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nAntenna-docker: Received bootstrap token");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nGetting state after receiving bootstrap tokens from cloud");
    sleep(Duration::from_secs(sleep_time)).await;

    // Show state after the devices gets their bootstrap tokens
    let cloud_bootstrap: MySecret = kv2::read(&cloud, "secret", "rotation/secret").await?;
    let bridge_bootstrap: MySecret = kv2::read(&antenna, "secret", "rotation/secret").await?;
    let antenna_bootstrap_docker: MySecret =
        kv2::read(&antenna, "secret", "rotation/secret").await?;
    let antenna_bootstrap_vault: MySecret =
        kv2::read(&antenna, "secret", "rotation/secret").await?;
    println!("\nState after receiving bootstrap tokens:");
    println!(
        "Cloud: {:?}\nBridge: {:?}\nAntenna_vault: {:?}\nAntenna_docker: {:?}",
        cloud_bootstrap, bridge_bootstrap, antenna_bootstrap_vault, antenna_bootstrap_docker
    );
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nGet data from cloud");
    let read: GetData = kv2::read(&cloud, "secret", "data").awaait?;
    println!(
        "name: {},\nage: {}, \nbirthday: {}",
        read.name, read.age, read.birthday
    );
    kv2::set(&antenna, "secret", "data", &data).await?;

    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nPush data to antenna vault");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nBridge requests token from antenna-vault");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nBridge receives token from antenna-vault");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nAntenna-docker requests token from antenna-vault");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nAntenna-docker receives token from antenna-vault");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nCloud makes new token");
    sleep(Duration::from_secs(sleep_time)).await;

    // Rotate secret in Central Vault
    kv2::set(
        &cloud,
        "secret",
        "rotation/secret",
        &MySecret {
            secret: "secret-token-v1".into(),
        },
    )
        .await?;
    // Cloud also stores the new antenna token
    kv2::set(
        &cloud,
        "secret",
        "rotation/antenna_token",
        &MySecret {
            secret: env::var("NEW_ANTENNA_VAULT_TOKEN")?,
        },
    )
        .await?;

    println!("\nGetting state before new access tokens are distributed from cloud");
    sleep(Duration::from_secs(2)).await;

    // Before rotation get new secrets from Central Vault and show state
    let cloud_new: MySecret = kv2::read(&cloud, "secret", "rotation/secret").await?;
    let bridge_before: MySecret = kv2::read(&antenna, "secret", "rotation/secret").await?;
    let antenna_before_docker: MySecret = kv2::read(&antenna, "secret", "rotation/secret").await?;
    let antenna_before_vault: MySecret = kv2::read(&antenna, "secret", "rotation/secret").await?;
    println!("\nState before new tokens");
    println!(
        "Cloud: {:?}\nBridge: {:?}\nAntenna_vault: {:?}\nAntenna_docker: {:?}",
        cloud_new, bridge_before, antenna_before_vault, antenna_before_docker
    );
    sleep(Duration::from_secs(sleep_time)).await;

    // Bridge: get new antenna token from cloud, push new secret with the OLD token,
    // then switch to the NEW token
    let new_antenna_token: MySecret =
        kv2::read(&cloud, "secret", "rotation/antenna_token").await?;
    kv2::set(&antenna, "secret", "rotation/secret", &cloud_new).await?;
    let antenna_new = vault_client(&antenna_addr, &new_antenna_token.secret)?;

    println!("\nBridge gets new token from cloud");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nBridge pushes new token to antenna-vault");

    println!("\nAntenna-vault receives new token");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nBridge pushes new token to antenna-docker");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nAntenna-docker receives new token");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nGetting state after distributing new tokens:");
    sleep(Duration::from_secs(sleep_time)).await;

    // After push get secrets from CLOUD and antenna vault with new access tokens
    let cloud_new: MySecret = kv2::read(&cloud, "secret", "rotation/secret").await?;
    let bridge_new: MySecret = kv2::read(&antenna_new, "secret", "rotation/secret").await?;
    let antenna_new_docker: MySecret =
        kv2::read(&antenna_new, "secret", "rotation/secret").await?;
    let antenna_new_vault: MySecret =
        kv2::read(&antenna_new, "secret", "rotation/secret").await?;
    println!("\nState after getting new tokens:");
    println!(
        "Cloud: {:?}\nBridge: {:?}\nAntenna_vault: {:?}\nAntenna_docker: {:?}",
        cloud_new, bridge_new, antenna_new_vault, antenna_new_docker
    );

    let cloud_old_token = vault_client(
        &env::var("CLOUD_VAULT_ADDR")?,
        &env::var("OLD_CLOUD_VAULT_TOKEN")?,
    )?;

    println!("\nTry to use bootstrap token to push:");
    sleep(Duration::from_secs(sleep_time)).await;

    let old_token_check: Result<(), Box<dyn std::error::Error>> = match kv2::set(
        &cloud_old_token,
        "secret",
        "rotation/secret",
        &MySecret {
            secret: "bootstrap-token-v0".into(),
        },
    )
        .await
    {
        Ok(_) => Err("\nold bootstrap token was still accepted, it should be rejected".into()),
        Err(ClientError::APIError { code: 403, .. }) => {
            println!("\nOld bootstrap token rejected error code (403), as expected");
            Ok(())
        }
        Err(ClientError::APIError { code: 503, .. }) => Err("\nvault sealed or unavailable".into()),
        Err(e) => Err(e.into()),
    };
    old_token_check?;
    sleep(Duration::from_secs(sleep_time)).await;

    // Status: check after deployment has passed
    println!("\nDeployment passed");
    sleep(Duration::from_secs(sleep_time)).await;

    println!("\nGet new data from cloud:");
    sleep(Duration::from_secs(sleep_time)).await;

    let read: GetData = kv2::read(&cloud, "secret", "data").await?;
    println!(
        "name: {},\nage: {},\nbirthday: {}",
        read.name, read.age, read.birthday
    );
    println!("\nPush to antenna-vault");
    sleep(Duration::from_secs(sleep_time)).await;

    loop {
        kv2::set(&antenna_new, "secret", "data", &data).await?;
        kv2::set(&cloud, "secret", "data", &data).await?;
        sleep(Duration::from_secs(sleep_time)).await;
        let read: GetData = kv2::read(&cloud, "secret", "data").await?;
        println!("\nname: {},\nage: {},\nbirthday: {}", read.name, read.age, read.birthday);
        break
    }

    println!("\nFor example this can loop every 30sec or on a specific time(s)\n");

    Ok(())
}