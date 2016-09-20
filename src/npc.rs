use inventory_item::InventoryItem;

// a non-player character
#[derive(Debug, Eq, PartialEq)]
pub struct NPC {
    pub name: String,
    pub inventory: Vec<InventoryItem>,
    pub dialogue: String,
}

impl NPC {
    pub fn new(name: String, inventory: Vec<InventoryItem>, dialogue: String) -> NPC {
        NPC {
            name: name,
            inventory: inventory,
            dialogue: dialogue,
        }
    }

    fn inventory(&self) -> &[InventoryItem] {
        &self.inventory[..]
    }

    pub fn inventory_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.inventory
    }
}
