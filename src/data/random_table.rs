pub const TABLE_TABLE_NAME: &str = "random_table";
pub const COLUMN_TABLE_ID: &str = "id";
pub const COLUMN_TABLE_NAME: &str = "name";
pub const COLUMN_TABLE_CREATED_BY: &str = "created_by";

pub const TABLE_ELEMENT_TABLE_NAME: &str = "random_table_element";
pub const COLUMN_TABLE_ELEMENT_INDEX: &str = "index";
pub const COLUMN_TABLE_ELEMENT_TABLE_ID: &str = "table_id";
pub const COLUMN_TABLE_ELEMENT_TEXT: &str = "text";

#[derive(Clone, Debug)]
pub struct Table {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub elements: Option<Vec<String>>,
}
