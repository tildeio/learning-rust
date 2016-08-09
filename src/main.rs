#![allow(dead_code)]
//#![feature(question_mark)]

use std::collections::HashMap;
use std::cmp::max;
use std::fmt;

pub type StringLiteral = &'static str;

#[derive(Debug, Eq, PartialEq)]
struct Player {
    name: StringLiteral,
    inventory: Vec<InventoryItem>,
    location: Location
}

impl Player {
    fn new(name: StringLiteral, inventory: Vec<InventoryItem>, x: u64, y: u64) -> Player {
        Player {
            name: name,
            inventory: inventory,
            location: Location { x: x, y: y }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InventoryItem {
    count: u64,
    name: String,
    effects: String
}

impl InventoryItem {
    fn new(count: u64, name: String, effects: String) -> InventoryItem {
        InventoryItem {
            count: count,
            name: name,
            effects: effects
        }
    }
}

// We want to be able to debug Location, `==` it (which requires
// Eq and PartialEq), use it as a HashMap key, and clone it.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Location {
    x: u64, // it has an x and y, both of which are unsigned numbers
    y: u64, // that means they cannot be negative (and we'll get an
}           // error if we try to underflow)

// The implementation is separate from the struct definition, which
// just describes the data structure. You can have as many `impl`
// blocks as you want, and they get merged together.
impl Location {
    // This provides Location::new(), which takes two `u64`s and
    // produces a `Location` object.
    fn new(x: u64, y: u64) -> Location {
        // Note that the lack of a `self` parameter is what makes
        // this a "class method", which means it is invoked as
        // Location::new(x, y).
        Location { x: x, y: y }
    }
}

// We want to be able to debug Room and `==` it.
#[derive(Debug, Eq, PartialEq)]
pub struct Room {
    location: Location,        // it has a Location
    name: String,              // it has a name
    description: String,       // it has a description
    items: Vec<InventoryItem>, // it has items/inventory (may be empty)
    npc: NPC                   // it may have an NPC / non-player character
}

impl Room {
    // As in Ruby, we make Room::new take an x, y and description,
    // and construct a Location using the x and y.
    fn new(x: u64, y: u64, name: String, description: String, items: Vec<InventoryItem>, npc: NPC) -> Room { // returns a Room
        Room { // Construct a Room
            location: Location::new(x, y),  // construct a Location
            name: name,                     // save the name
            description: description,       // save the description
            items: items,                   // save the inventory
            npc: npc                        // save the NPC
        }
    }
}

// a non-player character
#[derive(Debug, Eq, PartialEq)]
pub struct NPC {
    name: String,
    inventory: Vec<InventoryItem>,
    dialogue: String
}

impl NPC {
    fn new(name: String, inventory: Vec<InventoryItem>, dialogue: String) -> NPC {
        NPC {
            name: name,
            inventory: inventory,
            dialogue: dialogue
        }
    }
}

// We want to be able to debug Room and `==` it.
#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    title: String,
    rooms: HashMap<Location, Room>,
    max_x: u64, // east-most room
    max_y: u64  // north-most room
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidDirection {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

impl Map {
    fn new(title: StringLiteral, room_list: Vec<Room>) -> Map {
        // Make a new mutable HashMap
        let mut rooms = HashMap::new();

        // Make new mutable max_x and max_y values, initialized to 0
        let mut max_x = 0;
        let mut max_y = 0;

        // Iterate over the room_list
        for room in room_list {
            // extract x and y from the room's location
            let x = room.location.x;
            let y = room.location.y;

            // update max_x and max_y if necessary
            max_x = max(max_x, x);
            max_y = max(max_y, y);

            // insert the room into the rooms Hash with its location
            // as a key. clone the location because removing the
            // location from the room to use as a key prevents use
            // from using the room as a value.
            rooms.insert(room.location.clone(), room);
        }

        // construct a new Map
        Map {
            title: title.to_string(),
            rooms: rooms,
            max_x: max_x,
            max_y: max_y
        }
    }

    // valid_directions takes a Location contained in this
    // Map and answers which directions a player can go.
    // Specifically, a player is not allowed to move off
    // the edge of the map.
    fn valid_directions(&self, l: &Location) -> ValidDirection {
        ValidDirection {
            // identical to the Ruby code
            north: l.y < self.max_y,
            south: l.y > 0,
            east: l.x < self.max_x,
            west: l.x > 0
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // writeln!(_f, "{}", self.title)?;

        // for _ in self.title.chars() {
        //     write!(_f, "=")?;
        // }

        // write!(_f, "\n\n")?;

        // for row in &self.rooms {
        //     for room in row {
        //         write!(_f, "{:20} ", room.description)?;
        //     }

        //     write!(_f, "\n")?;

        //     for room in row {
        //         write!(_f, "{:20} ",
        //                format!("{}, {}", room.location.x, room.location.y))?;
        //     }

        //     write!(_f, "\n")?;
        // }

        Ok(())
    }
}

// A helper function for easily contructing a room to pass to Map::new.
// Used below in tests.
pub fn room(x: u64, y: u64, desc: StringLiteral) -> Room {
    // In Rust, string literals are "slices", which means they are
    // shared, but we want an owned String. We can use to_string()
    // to copy the StringLiteral into something we can own.
    Room::new(x, y, desc.to_string())
}

fn main() {
    // vec![] is the Array literal syntax in Rust.
    let rooms = vec![
        room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
        room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
        room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
    ];

    let map = Map::new("Liz's Great Adventure", rooms);
    let player = Player::new("Liz", vec![], 1, 1);

    println!("{:?}", map);
    println!("{:?}", player);
}

// cfg(test) means only include this code when compiling for test mode
#[cfg(test)]
mod tests {       // this is a nested module
    use super::*; // include all the public items from the parent module

    // helper function for constructing a 3x3 list of rooms for testing
    fn rooms() -> Vec<Room> {
        vec![
            room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
            room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
            room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
        ]
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
        assert_eq!(valid_directions, ValidDirection {
            north: true,
            south: false,
            east: true,
            west: false
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

        assert_eq!(valid_directions, ValidDirection {
            north: false,
            south: true,
            east: false,
            west: true
        });
    }

    #[test]
    fn valid_directions_for_top_right_in_big_map() {
        let map = big_map();
        let location = Location::new(3, 3);
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions, ValidDirection {
            north: false,
            south: true,
            east: false,
            west: true
        });
    }
}
