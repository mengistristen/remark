use std::{fs::File, io::Write, path::PathBuf};

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

    pub(crate) fn save(&self, path: &PathBuf) -> Result<(), RemarkError> {
        let frontmatter = serde_yaml::to_string(&self.metadata)?;
        let combined = format!("{}---\n{}", frontmatter, self.content);

        std::fs::write(path, combined)?;

        Ok(())
    }

    pub(crate) fn from_file(path: &PathBuf) -> Result<Self, RemarkError> {
        let contents = std::fs::read_to_string(path)?;

        Self::from_string(contents)
    }

    pub(crate) fn from_string(source: String) -> Result<Self, RemarkError> {
        if let Some((data, content)) = source.split_once("---\n") {
            let metadata = serde_yaml::from_str(data)?;

            Ok(Self::new(metadata, content.to_owned()))
        } else {
            Err(RemarkError::InvalidFile)
        }
    }

    pub(crate) fn as_writer(metadata: T, path: &PathBuf) -> Result<MdFileWriter, RemarkError> {
        let file = File::create(path)?;
        let mut writer = MdFileWriter { file };

        writer.write_metadata(&metadata)?;

        Ok(writer)
    }
}

pub(crate) struct MdFileWriter {
    file: File,
}

impl MdFileWriter {
    fn write_metadata<T>(&mut self, metadata: &T) -> Result<(), RemarkError>
    where
        T: Serialize,
    {
        let frontmatter = serde_yaml::to_string(metadata)?;
        writeln!(self.file, "{}---", frontmatter)?;
        Ok(())
    }
}

impl Write for MdFileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
