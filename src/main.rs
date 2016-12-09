#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod gameerror;
#[macro_use]
pub mod holder;
pub mod base;
pub mod actor;
pub mod room;
pub mod game;
pub mod terminal;

use game::*;
use actor::*;
use terminal::*;

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
