use serde::{Deserialize, Serialize};

pub async fn check_for_update(git_repo: String) -> anyhow::Result<String> {
    Ok(String::default())
}

#[derive(Serialize, Deserialize)]
struct ServerData {
    version: String,
}

pub async fn check_launcher_update() -> anyhow::Result<Option<String>> {
    println!("Checking server for launcher udpatte");
    let version = reqwest::Client::new()
        .post("http://rustipelago.dragmine.me/version.json")
        .send()
        .await?
        .json::<ServerData>()
        .await?;

    Ok((version.version != env!("CARGO_PKG_VERSION")).then(|| version.version))
}
