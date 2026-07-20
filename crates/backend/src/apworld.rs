//! Deal with handling zipping and unzipping apworlds.
//!
//! Mostly just borrowed code from: https://github.com/zip-rs/zip2/tree/master/examples

use anyhow::anyhow;
use rand::RngExt;
use rustipelago_schema::archipelago::{ApCard, ApWorldInfo, CardType};
use std::{
    env::temp_dir,
    fs::{File, Permissions, create_dir_all, remove_dir_all, set_permissions},
    io::Read,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, result::ZipError, write::SimpleFileOptions};

/// Save a folder of the apworld to the file system.
///
/// This is normally a build-from-source option, but as we get worlds via ref we expose this anyway.
///
/// # Arguments
/// - world_dir: The directory of ALL the files of the world.
/// - dest_dir: The directory of where to store the built world. Doesn't automatically add `.apworld` at the end.
pub fn write_folder(world_dir: PathBuf, dest_dir: PathBuf) -> anyhow::Result<()> {
    let zip_file = temp_dir().join(format!(
        "rustipelago/.{}.apworld",
        rand::rng().sample(rand::distr::Alphabetic) as char
    ));
    let file = File::create(&zip_file)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(world_dir.clone());
    for entry_result in walkdir.into_iter() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                return Err(anyhow!(
                    "Error while traversing directory {world_dir:?}: {e}"
                ));
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

/// Mount the zip file so that we can run the code without issue.
///
/// NOTE: We assume that the user has read/write permissions to their temp dir.
/// NOTE: The file passed in `world` does not autoappend `.apworld`
pub fn mount_world(world: &PathBuf, editing: bool) -> anyhow::Result<PathBuf> {
    let out_dir = temp_dir().join(format!(
        "rustipelago/.{}{}",
        rand::rng().sample(rand::distr::Alphabetic) as char,
        world.file_name().unwrap().display()
    ));
    _ = std::fs::create_dir_all(&out_dir)?;

    let mut archive = match File::open(&world)
        .map_err(ZipError::from)
        .and_then(ZipArchive::new)
    {
        Ok(archive) => archive,
        Err(e) => {
            eprintln!("Error: unable to open archive {:?}: {e}", world.display());
            return Err(e.into());
        }
    };

    let mut some_files_failed = false;
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: unable to open file {i} in archive: {e}");
                some_files_failed = true;
                continue;
            }
        };
        let out_path = (&out_dir).join(match file.enclosed_name() {
            Some(path) => path,
            None => {
                eprintln!(
                    "Error: unable to extract file {:?} because it has an invalid path.",
                    file.name()
                );
                some_files_failed = true;
                continue;
            }
        });

        if file.is_dir() {
            _ = create_dir_all(&out_path);
        } else {
            if let Some(p) = out_path.parent() {
                _ = create_dir_all(p);
            }
            match File::create(&out_path)
                .and_then(|mut outfile| std::io::copy(&mut file, &mut outfile))
            {
                Ok(bytes_extracted) => {
                    println!(
                        "File {} extracted to {:?} ({bytes_extracted} bytes)",
                        i,
                        out_path.display(),
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Error: unable to extract file {i} to {:?}: {e}",
                        out_path.display()
                    );
                    some_files_failed = true;
                    continue;
                }
            }
        }
        // Set the permissions to r-xr-xr-x as we don't really want to write and break stuff.
        // if let Err(e) = set_permissions(
        //     &out_path,
        //     Permissions::from_mode(if editing { 0o700 } else { 0o500 }),
        // ) {
        //     eprintln!(
        //         "Error: unable to change permissions of file {i} ({:?}): {e}",
        //         out_path.display()
        //     );
        //     some_files_failed = true;
        // }
    }

    if some_files_failed {
        eprintln!("Error: some files failed to extract; see above errors.");
        Err(anyhow!("Extraction partially failed"))
    } else {
        Ok(out_dir)
    }
}

/// Write the apworld back to the file and then remove the evidence.
pub fn unmount_world(
    world_dir: &PathBuf,
    world_mount: &PathBuf,
    world_name: &String,
) -> anyhow::Result<()> {
    write_folder(
        world_mount.to_owned(),
        world_dir.join(format!("{}.apworld", world_name)),
    )?;
    Ok(remove_dir_all(world_mount)?)
}

pub fn list_worlds(world_dir: &PathBuf) -> Vec<ApCard> {
    WalkDir::new(world_dir)
        .into_iter()
        .filter_map(|world| world.ok())
        .filter_map(|world| apcard_from_data(world.path()).ok())
        .collect()
}

/// Read the world data from the file system.
///
/// If this returns `None` something failed whilst reading the path and hence this world should be treated like it's corrupted.
pub fn read_ap_data<P>(world: P) -> anyhow::Result<ApWorldInfo>
where
    P: AsRef<Path>,
{
    Ok(serde_json::from_reader(
        ZipArchive::new(File::open(&world)?)?.by_name(&format!(
            "{}/archipelago.json",
            world.as_ref().file_stem().unwrap().display()
        ))?,
    )?)
}

pub fn apcard_from_data<P>(world: P) -> anyhow::Result<ApCard>
where
    P: AsRef<Path>,
{
    let data = read_ap_data(&world)?;
    Ok(ApCard {
        icon: None,
        name: data.game,
        python: Some(world.as_ref().to_path_buf()),
        // TODO: Read this from __init__.py
        card_type: CardType::Misc,
        ..Default::default()
    })
}
