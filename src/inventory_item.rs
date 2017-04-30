#[derive(Debug, Eq, PartialEq)]
pub struct InventoryItem {
    count: u64,
    pub name: String,
    pub effects: String,
}

impl InventoryItem {
    pub fn new(count: u64, name: String, effects: String) -> InventoryItem {
        InventoryItem {
            count: count,
            name: name,
            effects: effects,
        }
    }
}
