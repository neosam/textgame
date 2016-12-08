#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

use std::result;
#[macro_use]
mod holder;
use holder::*;

use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::fs::File;
use std::mem;

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));



#[derive(Debug)]
pub enum GameError {
    GeneralError(String)
}

impl fmt::Display for GameError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameError::GeneralError(ref msg) => write!(fmt, "{}", msg),
        }
    }
}
impl Error for GameError {
    fn description(&self) -> &str {
        match *self {
            GameError::GeneralError(ref msg) => msg,
        }
    }
}

type GameResult<T> = result::Result<T, GameError>;



impl Actor {
    pub fn new() -> Actor {
        Actor {
            name: "".to_string(),
            visible: true,
            display: false,
            health: Container {
                value: 0,
                max: 0
            }
        }
    }
}



impl Room {
    pub fn new() -> Room {
        Room {
            title: "".to_string(),
            description: "".to_string(),
            exits: Vec::new(),
            actors: Vec::new()
        }
    }
    pub fn with_title(title: String) -> Self {
        Room {
            title: title,
            description: "".to_string(),
            exits: Vec::new(),
            actors: Vec::new()
        }
    }

    pub fn add_exit(&mut self, exit: Exit) {
        self.exits.push(exit);
    }

    pub fn get_exit(&self, direction: String) -> Option<&Exit> {
        for exit in self.exits.iter() {
            if exit.label == direction {
                return Some(exit)
            }
        }
        None
    }
}

trait Watchable {
    fn watch(&self) -> String;
}

impl Game {
    pub fn new() -> Self {
        let mut room_holder: Holder<Room, RoomKey> = Holder::new();
        let mut actor_holder: Holder<Actor, ActorKey> = Holder::new();
        let room_ref = room_holder.add(Room::new());
        let actor_ref = actor_holder.add(Actor::new());
        Game {
            room_ref: room_ref,
            player_ref: actor_ref,
            rooms: room_holder,
            actors: actor_holder
        }
    }

    pub fn add_room(&mut self, room: Room) ->  RoomKey {
        self.rooms.add(room)
    }

    pub fn room_ref<'a>(&'a self, id: RoomKey) -> RoomGame<'a> {
        RoomGame {
            room: self.rooms.get(id),
            game: self
        }
    }
    pub fn room_mut<'a>(&'a mut self, id: RoomKey) -> RoomGameMut<'a> {
        RoomGameMut {
            room_key: id,
            game: self
        }
    }

    pub fn player_room<'a>(&'a self) -> RoomGame<'a> {
        self.room_ref(self.room_ref)
    }
    pub fn player_room_mut<'a>(&'a mut self) -> RoomGameMut<'a> {
        let room_ref = self.room_ref;
        self.room_mut(room_ref)
    }

    pub fn remove_actor_all_rooms(&mut self, key: ActorKey) {
        for room in self.rooms.items.iter_mut() {
            let new_actors = room.actors.iter()
                .map(|actor_ref| *actor_ref)
                .filter(|actor_ref| *actor_ref != key)
                .collect();
            room.actors = new_actors;
        }
    }
    pub fn warp_actor(&mut self, actor_key: ActorKey, room_key: RoomKey) {
        self.remove_actor_all_rooms(actor_key);
        self.rooms.get_mut(room_key).actors.push(actor_key);
    }
}

pub struct RoomGame<'a> {
    pub room: &'a Room,
    pub game: &'a Game
}
impl<'a> RoomGame<'a> {
    pub fn room_exits(&'a self) -> Box<Iterator<Item=&'a Exit> + 'a> {
        Box::new(self.room.exits.iter())
    }
    pub fn actors(&'a self) -> Box<Iterator<Item=(ActorKey, &'a Actor)> + 'a> {
        Box::new(
            self.room.actors
                .iter()
                .map(move | actor_key | (*actor_key, self.game.actors.get(*actor_key)))
        )
    }
}

pub struct RoomGameMut<'a> {
    pub room_key: RoomKey,
    pub game: &'a mut Game
}
impl<'a> RoomGameMut<'a> {
    pub fn room(&mut self) -> &mut Room {
        self.game.rooms.get_mut(self.room_key)
    }

    pub fn as_ref<'b>(&'b self) -> RoomGame<'b> {
        RoomGame::<'b> {
            room: self.game.rooms.get(self.room_key),
            game: self.game
        }
    }

    pub fn add_actor(&mut self, a: Actor) {
        let actor_key = self.game.actors.add(a);
        self.room().actors.push(actor_key)
    }
}

impl<'a> Watchable for RoomGame<'a> {
    fn watch(&self) -> String {
        let mut res = String::with_capacity(2048);
        res.push_str(&self.room.title);
        res.push_str("\n");
        res.push_str(&self.room.description);
        res.push_str("\n");
        res.push_str(
            &self.actors().fold(String::new(), | mut acc, (_, actor) | {
                acc.push_str(&actor.name.to_string()); acc
            })
        );
        res.push_str("\n");
        res.push_str(
            &self.room.exits.iter().fold(String::new(), | mut acc, exit | {
                acc.push_str(&exit.label);
                acc.push_str(" ");
                acc
            })
        );
        res
    }
}

type CommandFn = Box<Fn(&mut Game) -> result::Result<bool, Box<Error>>>;

pub fn input_string(prompt: &str) -> result::Result<String, Box<Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let line = line.trim();
    Ok(line.to_string())
}

pub fn read_multiline(prompt: &str, term: &str) -> result::Result<String, Box<Error>> {
    let mut res = String::new();
    loop {
        let string = input_string(prompt)?;
        if &string == term {
            break;
        }
        res.push_str(&string);
        res.push('\n');
    }
    Ok(res)
}

pub struct Terminal {
    pub game: Game,
    pub commands: HashMap<String, CommandFn>,
    pub prompt: String
}
impl Terminal {
    pub fn new(game: Game) -> Self {
        Terminal {
            game: game,
            commands: HashMap::new(),
            prompt: "> ".to_string()
        }
    }
    pub fn step(&mut self) -> result::Result<bool, Box<Error>> {
        let line = input_string(&self.prompt)?;
        if line == "" {
            return Ok((false))
        }
        let mut keywords = line.split(" ");
        let keyword = keywords.next().unwrap();
        if let Some(command) = self.commands.get(keyword) {
            command(&mut self.game)
        } else {
            Err(Box::new(GameError::GeneralError("Command not found".to_string())))
        }
    }
    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            match self.step() {
                Ok(true) => done = true,
                Err(ref err) => println!("{}", err),
                _ => {}
            }
        }
    }
}

pub fn cmd_look() -> CommandFn {
    Box::new(|game| {
        println!("{}", game.player_room().watch());
        Ok(false)
    })
}
pub fn cmd_room_ref() -> CommandFn {
    Box::new(|game| {
        println!("Room ID: {}", game.room_ref.get());
        Ok(false)
    })
}
pub fn cmd_add_exit() -> CommandFn {
    Box::new(|mut game| {
        let exit_label = input_string("Exit label: ")?;
        let room_id = input_string("Room key:  ")?;
        let room_id: usize = room_id.parse()?;
        game.player_room_mut().room().exits.push(Exit {
            label: exit_label,
            room_key: RoomKey::new(room_id)
        });
        Ok(false)
    })
}
pub fn cmd_add_room() -> CommandFn {
    Box::new(|mut game| {
        let title = input_string("Room title: ")?;
        let key = game.rooms.add(Room::with_title(title));
        println!("Room key: {}", key.get());
        Ok(false)
    })
}
pub fn cmd_move_player() -> CommandFn {
    Box::new(|mut game| {
        let direction = input_string("Exit name: ")?;
        let room_key = {
            let exit = game.player_room().room.get_exit(direction);
            if exit.is_none() {
                return Err(Box::new(GameError::GeneralError("Exit not found".to_string())));
            }
            let exit = exit.unwrap();
            let dest_room_key = exit.room_key;
            dest_room_key
        };
        let player_key = game.player_ref;
        game.warp_actor(player_key, room_key);
        game.room_ref = room_key;
        Ok(false)
    })
}
pub fn cmd_edit_room() -> CommandFn {
    Box::new(|mut game| {
        let title = input_string("Room title: ")?;
        println!("Type multiple lines and stop with the keyword end");
        let description = read_multiline("description: ", "end")?;
        let mut room = game.player_room_mut();
        let mut room = room.room();
        room.title = title;
        room.description = description;
        Ok(false)
    })
}

pub fn cmd_save() -> CommandFn {
    Box::new(|game| {
        println!("Enter name.  You may only use letters and digits.");
        let mut name = input_string("Game name: ")?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
            *c >= 'A' && *c <= 'Z' ||
            *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError("Found invalid characters".to_string())))
        } else {
            let string = serde_json::to_string(game)?;
            let mut out = File::create(name)?;
            write!(out, "{}", string)?;
            Ok(false)
        }
    })
}
pub fn cmd_load() -> CommandFn {
     Box::new(|game| {
        println!("Enter name.  You may only use letters and digits.");
        let mut name = input_string("Game name: ")?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
            *c >= 'A' && *c <= 'Z' ||
            *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError("Found invalid characters".to_string())))
        } else {
            let file = File::open(name)?;
            let mut load_game = serde_json::from_reader(file)?;
            mem::swap(&mut load_game, game);
            Ok(false)
        }
     })
}

pub fn cmd_quit() -> CommandFn {
    Box::new(|_| Ok(true))
}

fn main() {
    let mut game = Game::new();
    {
        let room_ref = game.room_ref;
        let mut game_room = game.room_mut(room_ref);
        game_room.room().title = "Test Room".to_string();
        game_room.room().description = "This is a room".to_string();
        let mut actor = Actor::new();
        actor.name = "Hero".to_string();
        game_room.add_actor(actor);
    }
    let mut terminal = Terminal::new(game);
    terminal.commands.insert("look".to_string(), cmd_look());
    terminal.commands.insert("quit".to_string(), cmd_quit());
    terminal.commands.insert("roomref".to_string(), cmd_room_ref());
    terminal.commands.insert("addexit".to_string(), cmd_add_exit());
    terminal.commands.insert("addroom".to_string(), cmd_add_room());
    terminal.commands.insert("go".to_string(), cmd_move_player());
    terminal.commands.insert("editroom".to_string(), cmd_edit_room());
    terminal.commands.insert("save".to_string(), cmd_save());
    terminal.commands.insert("load".to_string(), cmd_load());
    terminal.run();
}
