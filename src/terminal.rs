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

use room::*;
use base::*;
use base::Watchable;
use fight::DamageRes;
use lang::lang;

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
            Err(Box::new(GameError::GeneralError(lang().command_not_found(keyword))))
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
pub fn cmd_look_item() -> CommandFn {
    Box::new(|game| {
        let keyword = input_string(&lang().item_prompt())?;
        let room: &Room = game.player_room();
        let item = room.get_item(&keyword)
            .ok_or(lang().keyword_not_found(&keyword))?;
        println!("{}", item.watch());
        Ok(false)
    })
}
pub fn cmd_look_actor() -> CommandFn {
    Box::new(|game| {
        let keyword = input_string(&lang().actor_prompt())?;
        let room: &Room = game.player_room();
        let actor = room.get_actor(&keyword)
            .ok_or(lang().keyword_not_found(&keyword))?;
        println!("{}", actor.watch());
        Ok(false)
    })
}
pub fn cmd_room_ref() -> CommandFn {
    Box::new(|game| {
        println!("{}", lang().room_key_response(&game.room_ref));
        Ok(false)
    })
}
pub fn cmd_add_exit() -> CommandFn {
    Box::new(|mut game| {
        let exit_label = input_string(&lang().exit_label_prompt())?;
        let room_id = input_string(&lang().room_key_prompt())?;
        let room_id: usize = room_id.parse()?;
        game.player_room_mut().add_exit(Exit {
            label: exit_label,
            room_key: RoomKey::new(room_id)
        });
        Ok(false)
    })
}
pub fn cmd_add_room() -> CommandFn {
    Box::new(|mut game| {
        let title = input_string(&lang().room_title_prompt())?;
        let key = game.rooms.add(Room::with_title(title));
        println!("{}", lang().room_key_response(&key));
        Ok(false)
    })
}
pub fn cmd_move_player() -> CommandFn {
    Box::new(|mut game| {
        let direction = input_string(&lang().exit_name_prompt())?;
        let room_key = {
            let exit = game.player_room().get_exit(direction)
                .ok_or(lang().exit_not_found_response())?;
            let dest_room_key = exit.room_key;
            dest_room_key
        };
        let player_key = game.player_ref.clone();
        let from_room_key = game.room_ref;
        game.warp_actor(player_key, from_room_key, room_key)?;
        game.room_ref = room_key;
        Ok(false)
    })
}
pub fn cmd_take() -> CommandFn {
    Box::new(|mut game| {
        let item_key = input_string(&lang().item_prompt())?;
        let player_ref = game.player_ref.clone();
        game.player_room_mut().actor_take(
            &player_ref,
            &item_key
        )?;
        Ok(false)
    })
}
pub fn cmd_drop() -> CommandFn {
    Box::new(|mut game| {
        let item_key = input_string(&lang().item_prompt())?;
        let player_ref = game.player_ref.clone();
        game.player_room_mut().actor_drop(
            &player_ref,
            &item_key
        )?;
        Ok(false)
    })
}

pub fn cmd_attack() -> CommandFn {
    Box::new(|mut game| {
        let actor_key = input_string(&lang().actor_prompt())?;
        let player_ref = game.player_ref.clone();
        let mut room = game.player_room_mut();
        match room.attack(&player_ref, &actor_key)? {
            DamageRes::NoDamage =>
                println!("{}", lang().no_damage_response()),
            DamageRes::Default(damage) =>
                println!("{}", lang().damage_response(damage)),
            DamageRes::Dead =>
                println!("{}", lang().dead_response())
        };
        Ok(false)
    })
}

pub fn cmd_edit_room() -> CommandFn {
    Box::new(|mut game| {
        let title = input_string(&lang().room_title_prompt())?;
        println!("{}", lang().multiline_info());
        let description = read_multiline(
            &lang().description_prompt(),
            &lang().default_multiline_term())?;
        let mut room = game.player_room_mut();
        room.title = title;
        room.description = description;
        Ok(false)
    })
}

pub fn cmd_save() -> CommandFn {
    Box::new(|game| {
        println!("{}", lang().game_name_info());
        let mut name = input_string(&lang().game_name_prompt())?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
                *c >= 'A' && *c <= 'Z' ||
                *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError(lang().invalid_characters_response())))
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
        println!("{}", lang().game_name_info());
        let mut name = input_string(&lang().game_name_prompt())?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
                *c >= 'A' && *c <= 'Z' ||
                *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError(lang().invalid_characters_response())))
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