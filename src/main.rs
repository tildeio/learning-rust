#![allow(dead_code)]
#![feature(question_mark)]

pub type StringLiteral = &'static str;
// use std::io::Write;

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

#[derive(Debug, Eq, PartialEq)]
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
}

#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    title: String,
    rooms: Vec<Vec<Room>>,
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
    fn new(title: StringLiteral, room_list: Vec<Room>) -> Map {
        let mut rooms: Vec<Option<Vec<Option<Room>>>> = vec![];

        for room in room_list.iter() {
            let Location { x, y } = room.location;

            if rooms.len() > (x as usize) {
                if rooms[x] == None {
                    rooms[x] = Some(vec![])
                 }
            }
        }

        Map {
            title: title.to_string(),
            rooms: vec![],
        }
    }

    fn valid_directions(&self, l: &Location) -> ValidDirection {
        let (north, south) = match l.y {
            0 => (false, true),
            2 => (true, false),
            _ => (true, true)
        };

        let (west, east) = match l.x {
            0 => (false, true),
            2 => (true, false),
            _ => (true, true)
        };

        ValidDirection {
            north: north,
            south: south,
            east: east,
            west: west
        }
    }    
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}", self.title)?;

        for _ in self.title.chars() {
            write!(f, "=")?;
        }

        write!(f, "\n\n")?;

        for row in &self.rooms {
            for room in row {
                write!(f, "{:20} ", room.description)?;
            }

            write!(f, "\n")?;

            for room in row {
                write!(f, "{:20} ",
                       format!("{}, {}", room.location.x, room.location.y))?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn write(f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "hello")?;
    write!(f, "world")
}


// fn decrement(number: u64) -> Result<u64, StringLiteral> {
//     if number == 0 {
//         Err("Sorry, you cannot pass 0 to maybe_err")
//     } else {
//         Ok(number - 1)
//     }
// }

// fn decrement_three(x: u64, y: u64, z: u64) -> Result<(u64, u64, u64), StringLiteral> {
//     Ok((decrement(x)?, decrement(y)?, decrement(z)?))
// }

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

    println!("{:?}", player);

    // println!("{}", map);

    // // match decrement(0) {
    // //     Ok(number) => println!("{}", number),
    // //     Err(error) => println!("{}", error)
    // // }

    // match decrement_three(1, 5, 6) {
    //     Ok((x, y, z)) => println!("{}, {}, {}", x, y, z),
    //     Err(err) => println!("OMG!!!!! {}", err)
    // }

    // let x: Result<(), StringLiteral> = Err("hello").expect("I don't know why I'm so sure this can't fail...");

    // write!(std::io::stdout(), "hello world").expect("This can't be an error, and here's why");

    // let (number, is_ok) = maybe_error(1);
    // if is_ok {
    //     println!("{}", number);
    // } else {
    //     println!("Sorry, you cannot pass 0 to maybe_error");        
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> Map {
        let rooms = vec![
            room(0, 2, "top left"),    room(1, 2, "top center"),    room(2, 2, "top right"),
            room(0, 1, "middle left"), room(1, 1, "middle center"), room(2, 1, "middle right"),
            room(0, 0, "bottom left"), room(1, 0, "bottom center"), room(2, 0, "bottom right")
        ];

        Map::new("Liz's Great Adventure", rooms)
    }

    #[test]
    fn valid_directions_for_top_left() {
        let map = map();
        let location = Location { x: 0, y: 0 };
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions, ValidDirection {
            north: false,
            south: true,
            east: true,
            west: false 
        });
    }

    #[test]
    fn valid_directions_for_bottom_right() {
        let map = map();
        let location = Location { x: 2, y: 2 };
        let valid_directions = map.valid_directions(&location);

        assert_eq!(valid_directions, ValidDirection {
            north: true,
            south: false,
            east: false,
            west: true 
        });
    }

    
}