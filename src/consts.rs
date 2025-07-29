pub mod const_
{
    // Layout
    pub const MAX_COLUMNS: usize = 3;
    pub const COLUMNS: [&str; MAX_COLUMNS] = ["TODO", "IN PROGRESS", "DONE"];
    pub const ITEM_HEIGHT: u16 = 3;

    // Data
    pub const JSON: &str = "data.json";

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
}
