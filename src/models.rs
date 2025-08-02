pub mod model
{
    use serde::{Serialize, Deserialize};
    use crate::enums::enum_;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Configuration
    {
        pub theme: &'static str,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct TodoItem
    {
        pub title: String,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Model
    {
        pub col: usize,
        pub row: usize,
        pub items: Vec<Vec<TodoItem>>,
        pub running_state: enum_::RunningState,
        pub show_popup: bool,
        pub is_inputting: bool,
        pub input: String,
        pub popup_type: enum_::PopupType,
    }
}
