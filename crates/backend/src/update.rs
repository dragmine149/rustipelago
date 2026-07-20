use rustipelago_schema::ServerVersions;

/// Check to see if the launcher requires an update by sending a request to my server.
pub async fn get_server_versions() -> anyhow::Result<ServerVersions> {
    println!("Checking server for launcher udpatte");
    Ok(reqwest::Client::new()
        .post("https://rustipelago.dragmine.me/version.json")
        .send()
        .await?
        .json::<ServerVersions>()
        .await?)
}
