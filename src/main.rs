#![allow(dead_code)]
#![feature(question_mark)]

use std::collections::HashMap;
use std::cmp::max;

pub type StringLiteral = &'static str;

#[derive(Debug, Eq, PartialEq)]
struct Player {
    name: StringLiteral,
    location: Location
}

impl Player {
    fn new(name: StringLiteral, x: usize, y: usize) -> Player {
        Player {
            name: name,
            location: Location { x: x, y: y }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Location {
        Location { x: x, y: y }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Room {
    location: Location,
    description: String,
}

impl Room {
    fn new(x: usize, y: usize, description: StringLiteral) -> Room {
        Room {
            location: Location { x: x, y: y },
            description: description.to_string(),
        }
    }

    fn from_string(x: usize, y: usize, description: String) -> Room {
        Room {
            location: Location { x: x, y: y },
            description: description,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    title: String,
    rooms: HashMap<Location, Room>,
    max_x: usize,
    max_y: usize
}

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct ValidDirection {
    north: bool,
    south: bool,
    east: bool,
    west: bool
}

impl Map {
    fn new(title: StringLiteral, mut room_list: Vec<Room>) -> Map {
        let mut rooms = HashMap::new();

        let mut max_x = 0;
        let mut max_y = 0;

        for room in room_list.drain(..) {
            let x = room.location.x;
            let y = room.location.y;

            max_x = max(max_x, x);
            max_y = max(max_y, y);

            rooms.insert(room.location.clone(), room);
        }

        Map {
            title: title.to_string(),
            rooms: rooms,
            max_x: max_x,
            max_y: max_y
        }
    }

    fn valid_directions(&self, l: &Location) -> ValidDirection {
        ValidDirection {
            north: l.y < self.max_y,
            south: l.y > 0,
            east: l.x < self.max_x,
            west: l.x > 0
        }
    }    
}

impl fmt::Display for Map {
    fn fmt(&self, _f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // writeln!(f, "{}", self.title)?;

        // for _ in self.title.chars() {
        //     write!(f, "=")?;
        // }

        // write!(f, "\n\n")?;

        // for row in &self.rooms {
        //     for room in row {
        //         write!(f, "{:20} ", room.description)?;
        //     }

        //     write!(f, "\n")?;

        //     for room in row {
        //         write!(f, "{:20} ",
        //                format!("{}, {}", room.location.x, room.location.y))?;
        //     }

        //     write!(f, "\n")?;
        // }

        Ok(())
    }
}

pub fn room(x: usize, y: usize, desc: StringLiteral) -> Room {
    Room::new(x, y, desc)
}

fn main() {
    let rooms = vec![
        room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
        room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
        room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
    ];

    let map = Map::new("Liz's Great Adventure", rooms);
    let player = Player::new("Liz", 1, 1);

    println!("{:?}", map);
    println!("{:?}", player);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rooms() -> Vec<Room> {
        vec![
            room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
            room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
            room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
        ]
    }

    fn simple_room(x: usize, y: usize) -> Room {
      Room::from_string(x, y, format!("{}, {}", x, y))
    }

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

    #[test]
    fn valid_directions_for_bottom_left() {
        let map = map();
        let location = Location { x: 0, y: 0 };
        let valid_directions = map.valid_directions(&location);

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
        assert_eq!(map.rooms[&Location::new(0, 2)], rooms()[0]);
    }

    #[test]
    fn valid_directions_for_top_right() {
        let map = map();
        let location = Location { x: 2, y: 2 };
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
        let location = Location { x: 3, y: 3 };
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions, ValidDirection {
            north: false,
            south: true,
            east: false,
            west: true 
        });
    }
}