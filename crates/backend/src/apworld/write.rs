use anyhow::anyhow;
use rand::RngExt;
use std::{fs::File, path::PathBuf};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

pub fn write_apworld(world_dir: PathBuf, dest_dir: PathBuf) -> anyhow::Result<()> {
    let zip_file = PathBuf::from(format!(
        "/tmp/rustipelago/.{}.apworld",
        rand::rng().sample(rand::distr::Alphabetic) as char
    ));
    let file = File::create(&zip_file)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(world_dir.clone());
    for entry_result in walkdir.into_iter() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                return Err(anyhow!("Error while traversing directory {world_dir:?}: {e}").into());
            }
        };
        let path = entry.path();
        let path_stripped = path.strip_prefix(world_dir.clone())?;
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
    std::fs::rename(zip_file, dest_dir)?;
    Ok(())
}
