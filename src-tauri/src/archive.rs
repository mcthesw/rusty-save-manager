use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use zip::ZipWriter;
use zip_extensions::write::ZipWriterExtensions;

use crate::config::SaveUnit;

type BackupZip = HashMap<usize, String>; // Max size is usize

pub fn compress_to_file(
    save_paths: &Vec<SaveUnit>,
    backup_path: &PathBuf,
    date: &str,
) -> Result<()> {
    let file = File::create(backup_path.join([date, ".zip"].concat()))?;
    let mut zip = ZipWriter::new(file);
    let mut backup_map = BackupZip::new();
    save_paths.iter().enumerate().for_each(|(i, x)| {
        backup_map.insert(i, x.path.to_owned());
        //TODO: Add files to zip
    });
    zip.set_comment(serde_json::to_string_pretty(&backup_map)?); // Write the source path of file to the zip comment
    Ok(())
}
