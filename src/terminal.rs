use game::*;
use gameerror::*;
use holder::*;
use std::result;
use std::error::Error;
use std::io;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;
use std::mem;
use serde_json;

pub type CommandFn = Box<Fn(&mut Game) -> result::Result<bool, Box<Error>>>;

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