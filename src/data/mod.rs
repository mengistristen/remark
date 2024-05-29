use std::path::PathBuf;


use serde::{Deserialize, Serialize};

use crate::errors::RemarkError;

pub(crate) struct MdFile<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub(crate) metadata: T,
    pub(crate) content: String,
}

impl<T> MdFile<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    pub(crate) fn new(metadata: T, content: String) -> Self {
        Self { metadata, content }
    }

    pub(crate) fn save(&self, path: PathBuf) -> Result<(), RemarkError> {
        let frontmatter = serde_yaml::to_string(&self.metadata)?;
        let combined = format!("{}---\n{}", frontmatter, self.content);

        std::fs::write(path, combined)?;

        Ok(())
    }

    pub(crate) fn from_file(path: PathBuf) -> Result<Self, RemarkError> {
        let contents = std::fs::read_to_string(path)?;

        if let Some((data, content)) = contents.split_once("---\n") {
            let metadata = serde_yaml::from_str(data)?;

            Ok(Self::new(metadata, content.to_owned()))
        } else {
            Err(RemarkError::InvalidFile)
        }
    }
}
