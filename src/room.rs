use location::Location;
use inventory_item::InventoryItem;
use npc::NPC;

// We want to be able to debug Room and `==` it.
#[derive(Debug, Eq, PartialEq)]
pub struct Room {
    pub location: Location, // it has a Location
    pub name: String, // it has a name
    pub description: String, // it has a description
    pub items: Vec<InventoryItem>, // it has items/inventory (may be empty)
    pub npc: NPC, // it may have an NPC / non-player character
}

impl Room {
    // As in Ruby, we make Room::new take an x, y and description,
    // and construct a Location using the x and y.
    pub fn new(x: u64,
           y: u64,
           name: String,
           description: String,
           items: Vec<InventoryItem>,
           npc: NPC)
           -> Room {
        // returns a Room
        Room {
            // Construct a Room
            location: Location::new(x, y), // construct a Location
            name: name, // save the name
            description: description, // save the description
            items: items, // save the inventory
            npc: npc, // save the NPC
        }
    }

    pub fn items(&self) -> &[InventoryItem] {
        &self.items[..]
    }

    pub fn items_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.items
    }

    pub fn npc(&self) -> &NPC {
        &self.npc
    }

    pub fn npc_mut(&mut self) -> &mut NPC {
        &mut self.npc
    }
}