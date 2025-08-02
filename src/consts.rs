pub mod const_
{
    // Layout
    pub const MAX_COLUMNS: usize = 3;
    pub const COLUMNS: [&str; MAX_COLUMNS] = ["TODO", "IN PROGRESS", "DONE"];
    pub const ITEM_HEIGHT: u16 = 3;

    // Data
    pub const JSON_DATA: &str = "data.json";
    pub const JSON_CONFIG: &str = "config.json";

    // Style
    pub const GREEN: u8 = 2;
    pub const LIGHT_GREEN: u8 = 10;
    pub const YELLOW: u8 = 3;
    pub const LIGHT_YELLOW: u8 = 11;
    pub const WHITE: u8 = 7;
    pub const LIGHT_WHITE: u8 = 15;
    pub const BLACK: u8 = 0;
    pub const LIGHT_BLACK: u8 = 8;

    // Resource strings
    pub const RS_ADDITEM: &str = "Add Item";
    pub const RS_EDITITEM: &str = "Edit Item";
    pub const RS_HELP: &str = "Commands";
    pub const RS_HELP_COMMANDS: &str = "h: left
j: down
k: up
l: right

H: move issue left
J: move issue down
K: move issue up
L: move issue right

a: add new issue
e: edit selected issue
d: delete selected issue

?: show help message

q: quit
    ";
}
