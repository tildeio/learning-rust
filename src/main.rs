#![allow(dead_code)]
// #![feature(question_mark)]

extern crate regex;
use regex::Regex;

use std::collections::HashMap;
use std::cmp::max;
use std::fmt;
use std::io;

pub type StringLiteral = &'static str;

#[derive(Debug, Eq, PartialEq)]
struct Player {
    name: String,
    inventory: Vec<InventoryItem>,
    location: Location,
}

impl Player {
    fn new(inventory: Vec<InventoryItem>, x: u64, y: u64) -> Player {
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

    fn print_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Oops! You don't have any items. Why not take a look around?");
        } else {
            println!("{:?}", self.inventory);
        }
    }

    fn add_to_inventory(&mut self, item: InventoryItem) {
        println!("{} has been added to your inventory!", item.name);
        self.inventory.push(item);
    }

    fn inventory(&self) -> &[InventoryItem] {
        &self.inventory[..]
    }

    fn inventory_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.inventory
    }

    fn location(&self) -> &Location {
        &self.location
    }

    fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct InventoryItem {
    count: u64,
    name: String,
    effects: String,
}

impl InventoryItem {
    fn new(count: u64, name: String, effects: String) -> InventoryItem {
        InventoryItem {
            count: count,
            name: name,
            effects: effects,
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
    location: Location, // it has a Location
    name: String, // it has a name
    description: String, // it has a description
    items: Vec<InventoryItem>, // it has items/inventory (may be empty)
    npc: NPC, // it may have an NPC / non-player character
}

impl Room {
    // As in Ruby, we make Room::new take an x, y and description,
    // and construct a Location using the x and y.
    fn new(x: u64,
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

    fn items(&self) -> &[InventoryItem] {
        &self.items[..]
    }

    fn items_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.items
    }

    fn npc(&self) -> &NPC {
        &self.npc
    }

    fn npc_mut(&mut self) -> &mut NPC {
        &mut self.npc
    }
}

// a non-player character
#[derive(Debug, Eq, PartialEq)]
pub struct NPC {
    name: String,
    inventory: Vec<InventoryItem>,
    dialogue: String,
}

impl NPC {
    fn new(name: String, inventory: Vec<InventoryItem>, dialogue: String) -> NPC {
        NPC {
            name: name,
            inventory: inventory,
            dialogue: dialogue,
        }
    }

    fn inventory(&self) -> &[InventoryItem] {
        &self.inventory[..]
    }

    fn inventory_mut(&mut self) -> &mut Vec<InventoryItem> {
        &mut self.inventory
    }
}

// where most player console interactions and game loop will be defined
#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    player: Player,
    map: Map,
    playing: bool,
}

impl Game {
    fn start(player: Player, map: Map, playing: bool) {
        let mut game = Game::new(player, map, playing);
        game.play();
    }

    fn new(player: Player, map: Map, playing: bool) -> Game {
        println!("Hi {}Welcome to {}", player.name, map.title);

        Game {
            player: player,
            map: map,
            playing: playing,
        }
    }

    fn play(&mut self) {
        println!("What would you like to do?");
        self.parse_choice();
    }

    fn current_room(&self) -> &Room {
        self.map.rooms.get(&self.player.location).expect("BUG: The player's location must exist in the map")
    }

    fn current_room_mut(&mut self) -> &mut Room {
        self.map.rooms.get_mut(&self.player.location).expect("BUG: The player's location must exist in the map")
    }

    fn player(&self) -> &Player {
        &self.player
    }

    fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    fn parse_choice(&mut self) {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read line");

        let user_input = user_input.chomp();

        if user_input == "look around" {
            self.look_around();
        } else if user_input == "talk" {
            self.talk();
        } else if user_input == "display map" {
            self.map.display_map();
        } else if user_input == "print inventory" {
            self.player.print_inventory();
        } else if user_input == "exit" {
            println!("Thanks for playing!");
            self.playing = false;
        } else if user_input == "help" {
            println!("exit: exit the game");
            println!("move north, south, east, west: move in this direction");
            println!("look around: see a description of the current room");
            println!("pick up _item_: add the item to your inventory");
            println!("take _item_: take an item from an NPC");
            println!("use _item_: use an item in your inventory");
            println!("talk: talk to an NPC");
            println!("display map: look at map");
            println!("print inventory: show current player inventory");
        } else if let Some(captures) = regex("(?i)^pick up (?P<thing>.*)").captures(user_input) {
            self.pick_up(captures.name("thing").expect("unexpected optional capture"));
        } else if let Some(captures) = regex("(?i)^take (?P<thing>.*)").captures(user_input) {
            self.take(captures.name("thing").expect("unexpected optional capture"));
        } else if let Some(captures) = regex("(?i)^use (?P<thing>.*)").captures(user_input) {
            self.use_item(captures.name("thing").expect("unexpected optional capture"));
        } else if let Some(captures) = regex("(?i)^move (?P<direction>.*)").captures(user_input) {
            self.change_location(captures.name("direction").expect("unexpected optional capture"));
        } else {
            println!("{:?}", user_input);
            let split_input = user_input.split_whitespace();
            println!("{:?}", split_input.collect::<Vec<&str>>());
        }
    }

    // MOVES //

    fn string_to_inventory_item(&mut self, item: &str) -> Option<InventoryItem> {
        let items = self.current_room_mut().items_mut();

        items.iter()
            .position(|thing| item == thing.name)
            .map(|i| items.remove(i))
    }

    fn string_to_npc_item(&mut self, item_name: &str) -> Option<InventoryItem> {
        let npc_inventory = self.current_room_mut().npc_mut().inventory_mut();

        npc_inventory.iter()
            .position(|thing| item_name == thing.name)
            .map(|i| npc_inventory.remove(i))
    }

    fn string_to_player_item(&mut self, item_name: &str) -> Option<InventoryItem> {
        let player_inventory = self.player_mut().inventory_mut();

        player_inventory.iter()
            .position(|thing| item_name == thing.name)
            .map(|i| player_inventory.remove(i))
    }

    fn pick_up(&mut self, item_name: &str) {
        match self.string_to_inventory_item(item_name) {
            Some(item) => { self.player.add_to_inventory(item); }
            None => println!("Sorry, {} wasn't found in the current room", item_name) 
        };
    }

    fn take(&mut self, item_name: &str) {
        match self.string_to_npc_item(item_name) {
            Some(item) => { self.player.add_to_inventory(item); }
            None => println!("Sorry, {} doesn't have {}.", self.current_room().npc().name, item_name)
        }
    }

    fn use_item(&mut self, item_name: &str) {
        match self.string_to_player_item(item_name) {
            Some(item) => { println!("{}", item.effects) }
            None => println!("Sorry, you don't have {} in your inventory.", item_name)
        }
    }

    fn change_location(&mut self, direction: &str) {
        let valid_directions = self.map.valid_directions(&self.player().location());

        if direction == "north" {
            if valid_directions.north {
                self.player_mut().location_mut().y += 1;
                println!("You have moved north.");
            } else {
                println!("You can not go north. Try a different direction.");
            }
        } else if direction == "south" {
            if valid_directions.south {
                self.player_mut().location_mut().y -= 1;
                println!("You have moved south.");
            } else {
                println!("You can not go south. Try a different direction.");
            }
        } else if direction == "west" {
            if valid_directions.west {
                println!("You have moved west.");
                self.player_mut().location_mut().x -= 1;
            } else {
                println!("You can not go west. Try a different direction.");
            }
        } else if direction == "east" {
            if valid_directions.east {
                println!("You have moved east.");
                self.player_mut().location_mut().x += 1;
            } else {
                println!("You can not go north. Try a different direction.");
            }
        } else {
            println!("That is not a valid direction. Try north, south, east, or west.");
        }
    }

    fn look_around(&self) {
        // display the current room's description
        println!("{}", &self.current_room().description);
        // if the room has any items, display information about them
        if !&self.current_room().items.is_empty() {
            println!("This room contains: {:?}", &self.current_room().items);
        }
        // display information about the room's NPC
        println!("{} is here too!", &self.current_room().npc.name);

        if !&self.current_room().npc().inventory.is_empty() {
            println!("{} has {:?}.", &self.current_room().npc.name, &self.current_room().npc.inventory)
        }
    }

    fn talk(&self) {
        println!("{}", &self.current_room().npc.dialogue);
    }
}

fn regex(s: &str) -> Regex {
    Regex::new(s).unwrap()
}

// We want to be able to debug Room and `==` it.
#[derive(Debug, Eq, PartialEq)]
pub struct Map {
    title: String,
    rooms: HashMap<Location, Room>,
    max_x: u64, // east-most room
    max_y: u64, // north-most room
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidDirection {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
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
            max_y: max_y,
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
            west: l.x > 0,
        }
    }

    fn display_map(&self) {
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

// A helper function for easily contructing a room to pass to Map::new.
// Used below in tests.
pub fn room(x: u64,
            y: u64,
            name: StringLiteral,
            desc: StringLiteral,
            items: Vec<InventoryItem>,
            npc: NPC)
            -> Room {
    // In Rust, string literals are "slices", which means they are
    // shared, but we want an owned String. We can use to_string()
    // to copy the StringLiteral into something we can own.
    Room::new(x, y, name.to_string(), desc.to_string(), items, npc)
}

fn main() {
    // vec![] is the Array literal syntax in Rust.
    let rooms = vec![room(0,
                          2,
                          "top left",
                          "this is room one",
                          vec![InventoryItem::new(1, "cool potion".to_string(), "this potion has turned you into a C00L d00d!".to_string())],
                          NPC::new("George".to_string(), vec![], "hi I'm George".to_string())),
                     room(1,
                          2,
                          "top center",
                          "this is room two",
                          vec![InventoryItem::new(1, "dumb potion".to_string(), "this potion has turned you into a dumbb d00d!".to_string())],
                          NPC::new("Mike".to_string(), vec![], "hi I'm Mike".to_string())),
                     room(2,
                          2,
                          "top right",
                          "this is room three",
                          vec![InventoryItem::new(1, "stinky potion".to_string(), "this potion has turned you into a stinky d00d!".to_string())],
                          NPC::new("Helen".to_string(), vec![], "hi I'm Helen".to_string())),
                     room(0,
                          1,
                          "middle left",
                          "this is room four",
                          vec![InventoryItem::new(1, "charming potion".to_string(), "this potion has turned you into a charming d00d!".to_string())],
                          NPC::new("Linda".to_string(), vec![], "hi I'm Linda".to_string())),
                     room(1,
                          1,
                          "middle center",
                          "this is room five",
                          vec![InventoryItem::new(1, "dog potion".to_string(), "this potion has turned you into a C00L d0g!".to_string())],
                          NPC::new("Prudence".to_string(), vec![InventoryItem::new(1, "potato chip potion".to_string(), "this potion has given you potato chips. You can't eat them, but they're there. LOOKING AT YOU.".to_string())], "hi I'm Prudence".to_string())),
                     room(2,
                          1,
                          "middle right",
                          "this is room six",
                          vec![InventoryItem::new(1, "barfing potion".to_string(), "this potion has turned you into a barfing d00d!".to_string())],
                          NPC::new("Fred".to_string(), vec![], "hi I'm Fred".to_string())),
                     room(0,
                          0,
                          "bottom left",
                          "this is room seven",
                          vec![InventoryItem::new(1, "hungry potion".to_string(), "this potion has turned you into a hungry d00d!".to_string())],
                          NPC::new("Crocodile Man".to_string(), vec![], "hi I'm Crocodile Man".to_string())),
                     room(1,
                          0,
                          "bottom center",
                          "this is room eight",
                          vec![InventoryItem::new(1, "cute potion".to_string(), "this potion has turned you into a cute d00d!".to_string())],
                          NPC::new("Crocodile Woman".to_string(), vec![], "hi I'm Crocodile Woman".to_string())),
                     room(2,
                          0,
                          "bottom right",
                          "this is room nine",
                          vec![InventoryItem::new(1, "tall potion".to_string(), "this potion has turned you into a tall d00d!".to_string())],
                          NPC::new("Cool Unicorn".to_string(), vec![], "hi I'm Cool Unicorn".to_string()))];

    let map = Map::new("Liz's Great Adventure", rooms);
    let player = Player::new(vec![], 1, 1);
    let mut game = Game::new(player, map, true);

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