use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use flate2::{write::GzEncoder, Compression};
use tar::Builder;

use crate::{
    errors::RemarkError,
    utils::{self, RemarkDir},
};

pub fn process_export(output_file: String) -> Result<(), RemarkError> {
    let dirs = [
        utils::get_path(RemarkDir::Project)?,
        utils::get_path(RemarkDir::Task)?,
        utils::get_path(RemarkDir::Report)?,
    ];
    let base_path = utils::get_base_dir();

    let output_file_name = Path::new(&output_file)
        .file_name()
        .ok_or(RemarkError::Error("error getting file name".to_owned()))?
        .to_str()
        .ok_or(RemarkError::Error("error converting to string".to_owned()))?;

    let folder_name = if output_file.ends_with(".tar.gz") {
        &output_file_name[..output_file.len() - 7]
    } else {
        output_file_name
    };

    let tar_gz = File::create(&output_file)?;
    let encoder = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(encoder);

    for dir in dirs.iter() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let mut relative_path = PathBuf::from(folder_name);

                relative_path.push(path.strip_prefix(&base_path)?);
                tar.append_path_with_name(&path, relative_path)?;
            }
        }
    }

    tar.finish()?;

    Ok(())
}
