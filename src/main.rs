#![allow(dead_code)]
// #![feature(question_mark)]

pub mod player;
pub mod inventory_item;
pub mod location;
pub mod room;
pub mod npc;
pub mod game;
pub mod map;

pub type StringLiteral = &'static str;

// A helper function for easily contructing a room to pass to Map::new.
// Used below in tests.
pub fn room(x: u64,
            y: u64,
            name: StringLiteral,
            desc: StringLiteral,
            items: Vec<inventory_item::InventoryItem>,
            npc: npc::NPC)
            -> room::Room {
    // In Rust, string literals are "slices", which means they are
    // shared, but we want an owned String. We can use to_string()
    // to copy the StringLiteral into something we can own.
    room::Room::new(x, y, name.to_string(), desc.to_string(), items, npc)
}

fn main() {
    // vec![] is the Array literal syntax in Rust.
    let rooms = vec![room(0,
                          2,
                          "top left",
                          "this is room one",
                          vec![inventory_item::InventoryItem::new(1, "cool potion".to_string(), "this potion has turned you into a C00L d00d!".to_string())],
                          npc::NPC::new("George".to_string(), vec![], "hi I'm George".to_string())),
                     room(1,
                          2,
                          "top center",
                          "this is room two",
                          vec![inventory_item::InventoryItem::new(1, "dumb potion".to_string(), "this potion has turned you into a dumbb d00d!".to_string())],
                          npc::NPC::new("Mike".to_string(), vec![], "hi I'm Mike".to_string())),
                     room(2,
                          2,
                          "top right",
                          "this is room three",
                          vec![inventory_item::InventoryItem::new(1, "stinky potion".to_string(), "this potion has turned you into a stinky d00d!".to_string())],
                          npc::NPC::new("Helen".to_string(), vec![], "hi I'm Helen".to_string())),
                     room(0,
                          1,
                          "middle left",
                          "this is room four",
                          vec![inventory_item::InventoryItem::new(1, "charming potion".to_string(), "this potion has turned you into a charming d00d!".to_string())],
                          npc::NPC::new("Linda".to_string(), vec![], "hi I'm Linda".to_string())),
                     room(1,
                          1,
                          "middle center",
                          "this is room five",
                          vec![inventory_item::InventoryItem::new(1, "dog potion".to_string(), "this potion has turned you into a C00L d0g!".to_string())],
                          npc::NPC::new("Prudence".to_string(), vec![inventory_item::InventoryItem::new(1, "potato chip potion".to_string(), "this potion has given you potato chips. You can't eat them, but they're there. LOOKING AT YOU.".to_string())], "hi I'm Prudence".to_string())),
                     room(2,
                          1,
                          "middle right",
                          "this is room six",
                          vec![inventory_item::InventoryItem::new(1, "barfing potion".to_string(), "this potion has turned you into a barfing d00d!".to_string())],
                          npc::NPC::new("Fred".to_string(), vec![], "hi I'm Fred".to_string())),
                     room(0,
                          0,
                          "bottom left",
                          "this is room seven",
                          vec![inventory_item::InventoryItem::new(1, "hungry potion".to_string(), "this potion has turned you into a hungry d00d!".to_string())],
                          npc::NPC::new("Crocodile Man".to_string(), vec![], "hi I'm Crocodile Man".to_string())),
                     room(1,
                          0,
                          "bottom center",
                          "this is room eight",
                          vec![inventory_item::InventoryItem::new(1, "cute potion".to_string(), "this potion has turned you into a cute d00d!".to_string())],
                          npc::NPC::new("Crocodile Woman".to_string(), vec![], "hi I'm Crocodile Woman".to_string())),
                     room(2,
                          0,
                          "bottom right",
                          "this is room nine",
                          vec![inventory_item::InventoryItem::new(1, "tall potion".to_string(), "this potion has turned you into a tall d00d!".to_string())],
                          npc::NPC::new("Cool Unicorn".to_string(), vec![], "hi I'm Cool Unicorn".to_string()))];

    let map = map::Map::new("Liz's Great Adventure", rooms);
    let player = player::Player::new(vec![], 1, 1);
    let mut game = game::Game::new(player, map, true);

    while game.playing {
        game.play();
    }
}

// cfg(test) means only include this code when compiling for test mode
#[cfg(test)]
mod tests {
    // this is a nested module
    use super::*; // include all the public items from the parent module

    // helper function for constructing a 3x3 list of rooms for testing
    fn rooms() -> Vec<Room> {
        vec![room(0, 2, "top left"),
             room(1, 2, "top center"),
             room(2, 2, "top right"),
             room(0, 1, "middle left"),
             room(1, 1, "middle center"),
             room(2, 1, "middle right"),
             room(0, 0, "bottom left"),
             room(1, 0, "bottom center"),
             room(2, 0, "bottom right")]
    }

    // helper function for constructing a room whose description is
    // just its `x, y` coordinates.
    fn simple_room(x: u64, y: u64) -> Room {
        Room::new(x, y, format!("{}, {}", x, y))
    }

    // helper function for constructing a 4x4 list of rooms.
    fn big_rooms() -> Vec<Room> {
        vec![
        simple_room(0, 3), simple_room(1, 3), simple_room(2, 3), simple_room(3, 3),
        simple_room(0, 2), simple_room(1, 2), simple_room(2, 2), simple_room(3, 2),
        simple_room(0, 1), simple_room(1, 1), simple_room(2, 1), simple_room(3, 1),
        simple_room(0, 0), simple_room(1, 0), simple_room(2, 0), simple_room(3, 0),
      ]
    }

    fn map() -> Map {
        Map::new("Liz's Great Adventure", rooms())
    }

    fn big_map() -> Map {
        Map::new("Liz's Great Adventure", big_rooms())
    }

    #[test] // tell the test runner that this is a test function
    fn valid_directions_for_bottom_left() {
        let map = map();
        let location = Location { x: 0, y: 0 };
        let valid_directions = map.valid_directions(&location);

        // If the two sides are not equal, panic and fail the
        // test. This assumes both sides are Eq and PartialEq.
        assert_eq!(valid_directions,
                   ValidDirection {
                       north: true,
                       south: false,
                       east: true,
                       west: false,
                   });
    }

    #[test]
    fn looks_up_rooms_by_coordinates() {
        let map = map();
        // Indexing a HashMap does not take ownership of the value
        // passed to `[]`, so we lend the Location we create here
        // by using `&`.
        assert_eq!(map.rooms[&Location::new(0, 2)], rooms()[0]);
    }

    #[test]
    fn valid_directions_for_top_right() {
        let map = map();
        let location = Location::new(2, 2);
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions,
                   ValidDirection {
                       north: false,
                       south: true,
                       east: false,
                       west: true,
                   });
    }

    #[test]
    fn valid_directions_for_top_right_in_big_map() {
        let map = big_map();
        let location = Location::new(3, 3);
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions,
                   ValidDirection {
                       north: false,
                       south: true,
                       east: false,
                       west: true,
                   });
    }
}



trait Chomp {
    fn chomp(&self) -> &str;
}

impl Chomp for str {
    fn chomp(&self) -> &str {
        if self.ends_with('\n') {
            &self[0..self.len() - 1]
        } else {
            self
        }
    }
}

impl Chomp for String {
    fn chomp(&self) -> &str {
        self[..].chomp()
    }
}