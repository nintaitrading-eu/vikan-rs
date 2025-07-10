pub mod enum_
{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
    pub enum RunningState
    {
        #[default]
        Running,
        Done,
    }
}
