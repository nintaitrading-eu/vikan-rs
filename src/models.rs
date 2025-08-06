/*
 * model
 *     A collection of structs, that model the application.
 */
pub mod model
{
    use serde::{Serialize, Deserialize};
    use crate::enums::enum_;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Configuration
    {
        pub theme: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct TodoItem
    {
        pub title: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Theme
    {
        pub table: u8,
        pub table_light: u8,
        pub issue: u8,
        pub issue_light: u8,
        pub selected: u8,
        pub selected_light: u8,
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
