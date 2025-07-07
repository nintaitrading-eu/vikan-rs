pub mod error
{
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DataError
    {
        #[error("Data error.")]
        DataError,
    }

    impl From<std::io::Error> for DataError
    {
        fn from (_: std::io::Error) -> DataError
        {
            DataError::DataError
        }
    }
}
