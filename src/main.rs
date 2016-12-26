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
pub mod worldgen;

use game::*;
use actor::*;
use terminal::*;
use item::Item;
use lang::t;

fn main() {
    let mut game = Game::new();
    lang::init();
    println!("{}", t().welcome());
    match Terminal::new() {
        Ok(mut terminal) => {
            terminal.commands.insert(t().command_look(), cmd_look());
            terminal.commands.insert(t().command_lookitem(), cmd_look_item());
            terminal.commands.insert(t().command_lookactor(), cmd_look_actor());
            terminal.commands.insert(t().command_take(), cmd_take());
            terminal.commands.insert(t().command_drop(), cmd_drop());
            terminal.commands.insert(t().command_attack(), cmd_attack());
            terminal.commands.insert(t().command_quit(), cmd_quit());
            terminal.commands.insert("roomref".to_string(), cmd_room_ref());
            terminal.commands.insert("addexit".to_string(), cmd_add_exit());
            terminal.commands.insert("addroom".to_string(), cmd_add_room());
            terminal.commands.insert(t().command_go(), cmd_move_player());
            terminal.commands.insert("editroom".to_string(), cmd_edit_room());
            terminal.commands.insert(t().command_save(), cmd_save());
            terminal.commands.insert(t().command_load(), cmd_load());
            terminal.run();
        },
        Err(err) => println!("{}", err)
    }
}
