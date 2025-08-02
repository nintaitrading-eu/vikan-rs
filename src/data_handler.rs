pub mod data 
{
    use crate::config_handler::config;
    use crate::error_handler::error;
    use crate::models::model;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    pub fn load_data(model: &mut model::Model) -> Result<Option<model::Model>, error::ApplicationError>
    {
        config::ensure_config(model)?;

        let json_data = fs::read_to_string(config::get_output_file()).map_err(error::ApplicationError::IoError)?;
        let i: Vec<Vec<model::TodoItem>> = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
        model.items = i;
        Ok(Some(model.clone()))
    }

    pub fn save_data(model: &mut model::Model) -> Result<(), error::ApplicationError>
    {
        let json_data = serde_json::to_string_pretty(&model.items).unwrap();
        let mut file = File::create(config::get_output_file()).map_err(error::ApplicationError::IoError)?;
        file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;
        Ok(())
    }
}
