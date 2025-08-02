pub mod enum_
{
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
    pub enum RunningState
    {
        #[default]
        Running,
        Done,
    }

    #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
    pub enum PopupType
    {
        #[default]
        None,
        Add,
        Edit,
        Help,
    }
}
