/*
 * error_handler
 *     Custom error handling module.
 */
pub mod error
{
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ApplicationError
    {
        #[error("Error (de)serializing json.")]
        JsonError(serde_json::Error),

        #[error("Error (de)serializing json.")]
        SerdeError(serde_json::Error),

        #[error("IO error.")]
        IoError(#[from] std::io::Error),

        #[error("Unexpected error.")]
        UnexpectedError,
    }

    // Implement From for converting serde_json::Error to ApplicationError
    impl From<serde_json::Error> for ApplicationError
    {
        fn from(err: serde_json::Error) -> ApplicationError
        {
            ApplicationError::SerdeError(err)
        }
    }

    impl From<ApplicationError> for serde_json::Error
    {
        fn from(_err: ApplicationError) -> serde_json::Error
        {
           serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::Other, "Unexpected serde_json error.")) 
        }
    }
}
