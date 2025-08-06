/*
 * theme_handler
 *     Loads a color theme.
 *     To generate your own themes, create a json file, similar to default.json.
 *     Change the indexed color to your liking.
 *     For a list of the color values, see the indexed color values at
 *     https://ratatui.rs/examples/style/
 */
pub mod theme
{
    use crate::consts::const_;
    use crate::models::model;
    use crate::config_handler::config;
    use crate::error_handler::error;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn get_themes_dir() -> PathBuf
    {
        config::get_config_dir().as_path().join(const_::THEMES_DIR).to_path_buf()
    }

    pub fn get_default_theme_file() -> PathBuf
    {
        get_themes_dir().as_path().join(const_::DEFAULT_THEME_FILE).to_path_buf()
    }

    pub fn ensure_theme() -> Result<(), error::ApplicationError>
    {
        let theme_dir: PathBuf = get_themes_dir();
        if !theme_dir.exists()
        {
            println!("Theme directory does not exist yet, creating a default one at {:?}.", theme_dir);
            fs::create_dir_all(theme_dir.as_path()).map_err(error::ApplicationError::IoError)?;
        }

        let default_theme_file: PathBuf = get_default_theme_file();
        if !default_theme_file.exists()
        {
            println!("Default theme file does not exist yet, creating a one at {:?}.", Path::new(get_default_theme_file().as_path()));
            let default_theme = model::Theme
            {
                table: const_::GREEN,
                table_light: const_::LIGHT_GREEN,
                issue: const_::YELLOW,
                issue_light: const_::LIGHT_YELLOW,
                selected: const_::WHITE,
                selected_light: const_::LIGHT_WHITE,
                ..Default::default()
            };
            save(default_theme)?;
        }
        Ok(())
    }

    pub fn save(theme: model::Theme) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&theme).unwrap();
        let mut file = File::create(get_default_theme_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }

    pub fn load(config: model::Configuration) -> Result<model::Theme, error::ApplicationError>
    {
        let mut theme_file: PathBuf = get_themes_dir().as_path().join([config.theme, const_::EXT.to_string()].join("."));
        if !theme_file.exists()
        {
            println!("Warning: Theme {:?} not found. Falling back on default theme.", theme_file.as_path());
            theme_file = get_default_theme_file();
        }
        let json_data = fs::read_to_string(theme_file).map_err(error::ApplicationError::IoError)?;
        let i: model::Theme = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        Ok(i.clone())
    }
}
