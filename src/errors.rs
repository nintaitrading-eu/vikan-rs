pub mod error
{
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ApplicationError
    {
        /*#[error("Error deserializing json.")]
        JsonDeserializeError(#[from] serde_json::Error),
        
        #[error("Error serializing json.")]
        JsonSerializeError(#[from] serde_json::Error),
       */ 
        #[error("Unexpected error.")]
        UnexpectedError(#[from] std::io::Error),
    }
}
