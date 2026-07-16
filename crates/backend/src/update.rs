use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ServerData {
    version: String,
}

/// Check to see if the launcher requires an update by sending a request to my server.
pub async fn check_launcher_update() -> anyhow::Result<Option<String>> {
    println!("Checking server for launcher udpatte");
    let version = reqwest::Client::new()
        .post("https://rustipelago.dragmine.me/version.json")
        .send()
        .await?
        .json::<ServerData>()
        .await?;

    Ok((version.version != env!("CARGO_PKG_VERSION")).then_some(version.version))
}
