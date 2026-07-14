use std::{fs::File, path::PathBuf};

use anyhow::anyhow;
use ghgrab::{
    download::{self, Downloader},
    github::{GitHubClient, GitHubUrl, GitTreeEntry, RepoItem},
};
use reqwest::Client;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

use crate::apworld;

pub enum GithubInstallSource {
    Release,
    Ref,
}

pub async fn install_apworld_from_github(
    github: String,
    branch: String,
    world_name: String,
    source: GithubInstallSource,
    worlds_dir: PathBuf,
) -> anyhow::Result<()> {
    match source {
        // GithubInstallSource::Release => format!(
        //     "https://github.com/{}/releases/latest/download/{}.apworld",
        //     github, world_name
        // ),
        GithubInstallSource::Ref => gh_folder(github, world_name, worlds_dir).await,
        _ => todo!(),
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
        .map(|item| item.clone())
        .collect::<Vec<RepoItem>>();
    let dest_folder = PathBuf::from(format!("/tmp/rustipelago/{}", world_name));
    let downloader = Downloader::new(dest_folder.clone(), gh)?;
    downloader
        .download_items(&world_data, "", |progress| println!("{}", progress))
        .await?;

    apworld::write::write_apworld(
        dest_folder,
        worlds_dir.join(format!("{world_name}.apworld")),
    )
}
