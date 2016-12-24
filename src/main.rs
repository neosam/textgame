#![feature(proc_macro)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod lang;
pub mod gameerror;
#[macro_use]
pub mod holder;
pub mod base;
pub mod fight;
pub mod item;
pub mod actor;
pub mod room;
pub mod game;
pub mod terminal;

use game::*;
use actor::*;
use terminal::*;
use item::Item;
use lang::t;

fn main() {
    let mut game = Game::new();
    {
        let room_ref = game.room_ref;
        let mut game_room = game.room_mut(room_ref);
        game_room.title = "Test Room".to_string();
        game_room.description = "This is a room".to_string();
        let mut actor = Actor::default();
        actor.keyword = "you".to_string();
        actor.name = "Hero".to_string();
        actor.attributes.attack.value = 20;
        game_room.add_actor(actor);
        let mut actor2 = Actor::default();
        actor2.keyword = "somebody".to_string();
        actor2.name = "Somebody".to_string();
        actor2.attributes.defence.value = 15;
        actor2.health.max = 100;
        actor2.health.value = 10;
        game_room.add_actor(actor2);
        let mut item = Item::default();
        item.keyword = "doll".to_string();
        item.label = "A doll".to_string();
        item.description = "A beautiful doll".to_string();
        game_room.add_item(item);
    }
    lang::init();
    println!("{}", t().welcome());
    let mut terminal = Terminal::new(game);
    terminal.commands.insert("look".to_string(), cmd_look());
    terminal.commands.insert("lookitem".to_string(), cmd_look_item());
    terminal.commands.insert("lookactor".to_string(), cmd_look_actor());
    terminal.commands.insert("take".to_string(), cmd_take());
    terminal.commands.insert("drop".to_string(), cmd_drop());
    terminal.commands.insert("attack".to_string(), cmd_attack());
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
