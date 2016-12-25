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
use lang::t;

static DEFAULT_AREA_WIDTH: u32 = 5;
static DEFAULT_AREA_HEIGHT: u32 = 5;

use worldgen::worldgen::WorldGen;

pub type CommandFn = Box<Fn(CommandArg) -> result::Result<bool, Box<Error>>>;

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
    pub world_gen: WorldGen,
    pub commands: HashMap<String, CommandFn>,
    pub prompt: String
}

pub struct CommandArg<'a> {
    pub game: &'a mut Game,
    pub world_gen: &'a mut WorldGen
}

impl Terminal {
    pub fn new(game: Game) -> Self {
        Terminal {
            game: game,
            commands: HashMap::new(),
            prompt: "> ".to_string(),
            world_gen: WorldGen::new((DEFAULT_AREA_WIDTH, DEFAULT_AREA_HEIGHT))
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
            let args = CommandArg {
                game: &mut self.game,
                world_gen: &mut self.world_gen
            };
            command(args)
        } else {
            Err(Box::new(GameError::GeneralError(t().command_not_found(keyword))))
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
    Box::new(|term| {
        println!("{}", term.game.player_room().watch());
        Ok(false)
    })
}
pub fn cmd_look_item() -> CommandFn {
    Box::new(|term| {
        let keyword = input_string(&t().item_prompt())?;
        let room: &Room = term.game.player_room();
        let item = room.get_item(&keyword)
            .ok_or(t().keyword_not_found(&keyword))?;
        println!("{}", item.watch());
        Ok(false)
    })
}
pub fn cmd_look_actor() -> CommandFn {
    Box::new(|term| {
        let keyword = input_string(&t().actor_prompt())?;
        let room: &Room = term.game.player_room();
        let actor = room.get_actor(&keyword)
            .ok_or(t().keyword_not_found(&keyword))?;
        println!("{}", actor.watch());
        Ok(false)
    })
}
pub fn cmd_room_ref() -> CommandFn {
    Box::new(|term| {
        println!("{}", t().room_key_response(&term.game.room_ref));
        Ok(false)
    })
}
pub fn cmd_add_exit() -> CommandFn {
    Box::new(|mut term| {
        let exit_label = input_string(&t().exit_label_prompt())?;
        let room_id = input_string(&t().room_key_prompt())?;
        let room_id: usize = room_id.parse()?;
        term.game.player_room_mut().add_exit(Exit {
            label: exit_label,
            room_key: RoomKey::new(room_id)
        });
        Ok(false)
    })
}
pub fn cmd_add_room() -> CommandFn {
    Box::new(|mut term| {
        let title = input_string(&t().room_title_prompt())?;
        let key = term.game.rooms.add(Room::with_title(title));
        println!("{}", t().room_key_response(&key));
        Ok(false)
    })
}
pub fn cmd_move_player() -> CommandFn {
    Box::new(|mut term| {
        let mut game = &mut term.game;
        let direction = input_string(&t().exit_name_prompt())?;
        let room_key = {
            let exit = game.player_room().get_exit(direction)
                .ok_or(t().exit_not_found_response())?;
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
    Box::new(|mut term| {
        let mut game = &mut term.game;
        let item_key = input_string(&t().item_prompt())?;
        let player_ref = game.player_ref.clone();
        game.player_room_mut().actor_take(
            &player_ref,
            &item_key
        )?;
        Ok(false)
    })
}
pub fn cmd_drop() -> CommandFn {
    Box::new(|mut term| {
        let mut game = &mut term.game;
        let item_key = input_string(&t().item_prompt())?;
        let player_ref = game.player_ref.clone();
        game.player_room_mut().actor_drop(
            &player_ref,
            &item_key
        )?;
        Ok(false)
    })
}

pub fn cmd_attack() -> CommandFn {
    Box::new(|mut term| {
        let mut game = &mut term.game;
        let actor_key = input_string(&t().actor_prompt())?;
        let player_ref = game.player_ref.clone();
        let mut room = game.player_room_mut();
        match room.attack(&player_ref, &actor_key)? {
            DamageRes::NoDamage =>
                println!("{}", t().no_damage_response()),
            DamageRes::Default(damage) =>
                println!("{}", t().damage_response(damage)),
            DamageRes::Dead =>
                println!("{}", t().dead_response())
        };
        Ok(false)
    })
}

pub fn cmd_edit_room() -> CommandFn {
    Box::new(|mut term| {
        let mut game = &mut term.game;
        let title = input_string(&t().room_title_prompt())?;
        println!("{}", t().multiline_info());
        let description = read_multiline(
            &t().description_prompt(),
            &t().default_multiline_term())?;
        let mut room = game.player_room_mut();
        room.title = title;
        room.description = description;
        Ok(false)
    })
}

pub fn cmd_save() -> CommandFn {
    Box::new(|term| {
        let game = &term.game;
        println!("{}", t().game_name_info());
        let mut name = input_string(&t().game_name_prompt())?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
                *c >= 'A' && *c <= 'Z' ||
                *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError(t().invalid_characters_response())))
        } else {
            let string = serde_json::to_string(game)?;
            let mut out = File::create(name)?;
            write!(out, "{}", string)?;
            Ok(false)
        }
    })
}
pub fn cmd_load() -> CommandFn {
    Box::new(|mut term| {
        let mut game = term.game;
        println!("{}", t().game_name_info());
        let mut name = input_string(&t().game_name_prompt())?;
        let failed_validation = name.chars().find(|c|
            !(*c >= 'a' && *c <= 'z' ||
                *c >= 'A' && *c <= 'Z' ||
                *c >= '0' && *c <= '9'));
        name.push_str(".json");
        if failed_validation.is_some() {
            Err(Box::new(GameError::GeneralError(t().invalid_characters_response())))
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