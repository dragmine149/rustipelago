use reqwest::Client;

pub async fn check_for_update(http_client: Client, git_repo: String) -> anyhow::Result<String> {
    Ok(String::default())
}
