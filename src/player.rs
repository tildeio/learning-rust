use std::io;
use inventory_item::InventoryItem;
use location::Location;

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub name: String,
    inventory: Vec<InventoryItem>,
    pub location: Location,
}

impl Player {
    pub fn new(inventory: Vec<InventoryItem>, x: u64, y: u64) -> Player {
        // get user input an assign the input to `name`
        println!("What's your name?");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Could not read line");

        Player {
            name: name,
            inventory: inventory,
            location: Location { x: x, y: y },
        }
    }

    pub fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Oops! You don't have any items. Why not take a look around?");
        } else {
            println!("{:?}", self.inventory);
        }
    }

    pub fn add_to_inventory(&mut self, item: InventoryItem) {
        println!("{} has been added to your inventory!", item.name);
        self.inventory.push(item);
    }

    pub fn inventory(&self) -> &[InventoryItem] {
        &self.inventory[..]
    }

    pub fn inventory_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.inventory
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}
