use std::{fs::File, path::PathBuf};

use anyhow::anyhow;
use ghgrab::{
    download::{self, Downloader},
    github::{GitHubClient, GitHubUrl, GitTreeEntry, RepoItem},
};
use reqwest::Client;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

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

    let zip_file = PathBuf::from(format!("/tmp/rustipelago/.fhewgio{}.apworld", world_name));
    let file = File::create(&zip_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(dest_folder.clone());
    for entry_result in walkdir.into_iter() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                return Err(
                    anyhow!("Error while traversing directory {dest_folder:?}: {e}").into(),
                );
            }
        };
        let path = entry.path();
        let path_stripped = path.strip_prefix(dest_folder.clone())?;
        let path_as_string = path_stripped
            .to_str()
            .map(str::to_owned)
            .ok_or_else(|| anyhow!("{:?} is a Non UTF-8 Path", path_stripped.display()))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!(
                "adding file {:?} as {:?} ...",
                path.display(),
                path_stripped.display()
            );
            zip.start_file(path_as_string, options)?;
            let mut f = File::open(path)?;

            std::io::copy(&mut f, &mut zip)?;
        } else if !path_stripped.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!(
                "adding dir '{}' as '{}' ...",
                path.display(),
                path_stripped.display()
            );
            zip.add_directory(path_as_string, options)?;
        }
    }
    zip.finish()?;
    std::fs::rename(zip_file, worlds_dir.join("{world_name}.apworld"))?;

    Ok(())
}
