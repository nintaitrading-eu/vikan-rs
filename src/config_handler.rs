pub mod config
{
    use crate::consts::const_;
    use crate::models::model;
    use crate::error_handler::error;
    use crate::data_handler::data;
    use directories_next::ProjectDirs;
    use std::path::{Path, PathBuf};
    use std::fs;

    fn get_config_dir() -> PathBuf
    {
        match ProjectDirs::from("com", "nintaitrading", "vikan")
        {
            Some(proj_dirs) => proj_dirs.config_dir().to_path_buf(),
            _ => Path::new(".").to_path_buf(),
        }
    }

    pub fn get_output_file() -> PathBuf
    {
        get_config_dir().as_path().join(const_::JSON).to_path_buf()
    }

    pub fn ensure_config(model: &mut model::Model) -> Result<(), error::ApplicationError>
    {
        let config_dir: PathBuf = get_config_dir();
        if !config_dir.exists()
        {
            println!("Configuration directory does not exist yet, creating a default one at {:?}.", config_dir);
            fs::create_dir_all(config_dir.as_path()).map_err(error::ApplicationError::IoError)?;
        }

        let output_file: PathBuf = get_output_file();
        if !output_file.exists()
        {
            data::save_data(model)?;
            println!("Output file does not exist yet, creating a default one at {:?}.", Path::new(config_dir.as_path()).join(output_file.as_path()));
        }
        Ok(())
    }
}
