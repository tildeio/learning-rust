use player::Player;
use map::Map;
use room::Room;
use inventory_item::InventoryItem;
use std::io;

extern crate regex;
use Chomp;

// where most player console interactions and game loop will be defined
#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    player: Player,
    map: Map,
    pub playing: bool,
}

impl Game {
    fn start(player: Player, map: Map, playing: bool) {
        let mut game = Game::new(player, map, playing);
        game.play();
    }

    pub fn new(player: Player, map: Map, playing: bool) -> Game {
        println!("Hi {}Welcome to {}", player.name, map.title);
		println!("What would you like to do? (Enter 'help' to see a list of commands)");

        Game {
            player: player,
            map: map,
            playing: playing,
        }
    }

    pub fn play(&mut self) {
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

fn regex(s: &str) -> regex::Regex {
    regex::Regex::new(s).unwrap()
}
