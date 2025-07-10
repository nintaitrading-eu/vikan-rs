pub mod model
{
    use serde::{Serialize, Deserialize};
    use crate::enums::enum_;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct TodoItem
    {
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct Model
    {
        pub col: usize,
        pub row: usize,
        pub items: Vec<Vec<TodoItem>>,
        pub running_state: enum_::RunningState,
        pub show_popup: bool,
        pub inputting: bool,
        pub input: String,
    }
}
