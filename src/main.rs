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

    let cloud = make_client("http://127.0.0.1:8200", "cloud");
    let antenna = make_client("http://127.0.0.1:8210", "antenna");

    kv2::set(&cloud, "secret", "test", &MySecret { secret: "from cloud".into() }).await?;

    kv2::set(&antenna, "secret", "test", &MySecret { secret: "from antenna".into() }).await?;

    let c: MySecret = kv2::read(&cloud, "secret", "test").await?;
    let a: MySecret = kv2::read(&antenna, "secret", "test").await?;
    println!("Cloud: {:?}\nAntenna: {:?}", c, a);

    Ok(())
}
