use std::fs;

pub enum DataDir {
    Project,
    Task,
}

fn get_base_dir() -> std::path::PathBuf {
    let mut data_dir = dirs::data_local_dir().expect("failed to find data directory");

    data_dir.push("renew");
    fs::create_dir_all(&data_dir).expect("failed to create data directory");

    data_dir
}

pub fn get_project_path(dir: DataDir) -> Result<std::path::PathBuf, std::io::Error> {
    let base_path = get_base_dir();
    let path = match dir {
        DataDir::Project => base_path.join("projects"),
        DataDir::Task => base_path.join("tasks"),
    };

    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }

    Ok(path)
}
