use crate::apworld;
use ghgrab::{
    download::Downloader,
    github::{GitHubClient, RepoItem},
};
use std::path::PathBuf;

pub enum GithubInstallSource {
    Release,
    Ref,
}

pub async fn install_apworld_from_github(
    github: String,
    world_name: String,
    source: GithubInstallSource,
    worlds_dir: PathBuf,
) -> anyhow::Result<()> {
    match source {
        GithubInstallSource::Release => gh_releases(github, world_name, worlds_dir).await,
        GithubInstallSource::Ref => gh_folder(github, world_name, worlds_dir).await,
    }
}

async fn gh_folder(github: String, world_name: String, worlds_dir: PathBuf) -> anyhow::Result<()> {
    let gh = GitHubClient::new(None)?;
    let tree = gh
        .fetch_contents(&format!("https://github.com/{}", github))
        .await?;
    let world_path = format!("worlds/{}", world_name);
    let world_data = tree
        .iter()
        .filter(|item| item.path.contains(&world_path))
        .cloned()
        .collect::<Vec<RepoItem>>();
    let dest_folder = PathBuf::from(format!("/tmp/rustipelago/{}", world_name));
    let downloader = Downloader::new(dest_folder.clone(), gh)?;
    downloader
        .download_items(&world_data, "", |progress| println!("{}", progress))
        .await?;

    apworld::write_folder(
        dest_folder,
        worlds_dir.join(format!("{world_name}.apworld")),
    )
}

async fn gh_releases(
    github: String,
    world_name: String,
    worlds_dir: PathBuf,
) -> anyhow::Result<()> {
    let release = reqwest::Client::new()
        .get(format!(
            "https://github.com/{}/releases/latest/download/{}.apworld",
            github, world_name
        ))
        .send()
        .await?;
    let file_path = worlds_dir.join(format!("{world_name}.apworld"));
    let bytes = release.bytes().await?;
    std::fs::write(file_path, bytes)?;
    Ok(())
}
