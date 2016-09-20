use std::collections::HashMap;
use std::cmp::max;
use std::fmt;
use location::Location;
use room::Room;

pub type StringLiteral = &'static str;

// We want to be able to debug Room and `==` it.
#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    pub title: String,
    pub rooms: HashMap<Location, Room>,
    max_x: u64, // east-most room
    max_y: u64, // north-most room
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidDirection {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

impl Map {
    pub fn new(title: StringLiteral, room_list: Vec<Room>) -> Map {
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
            max_y: max_y,
        }
    }

    // valid_directions takes a Location contained in this
    // Map and answers which directions a player can go.
    // Specifically, a player is not allowed to move off
    // the edge of the map.
    pub fn valid_directions(&self, l: &Location) -> ValidDirection {
        ValidDirection {
            // identical to the Ruby code
            north: l.y < self.max_y,
            south: l.y > 0,
            east: l.x < self.max_x,
            west: l.x > 0,
        }
    }

    pub fn display_map(&self) {
        println!("Possible Destinations for {}", self.title);
        println!("{:=<1$}", "", self.title.len() + 26);
        for room_info in &self.rooms {
            println!("{}", room_info.1.name);
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