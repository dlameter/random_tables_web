#[derive(Clone, Debug)]
pub struct Table {
    pub id: i32,
    pub created_by: i32,
    pub name: String,
    pub elements: Vec<String>,
}
