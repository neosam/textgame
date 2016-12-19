use worldgen::context;
use worldgen::roomgen::RoomGen;
use room::Room;
use base::RoomKey;
use std::result::Result;
use std::error::Error;
use game::Game;

pub struct City;

impl RoomGen for City {
    fn gen_room(&mut self,
                game: &mut Game,
                area: &context::Area,
                pos: &context::Pos,
                null_room_key: RoomKey) -> Result<context::RoomDef, Box<Error>> {
        let mut room = Room::default();
        room
            .title("A city room")
            .description("You are in the big city.");
        let room_key = game.add_room(room);
        Ok(context::RoomDef {
            exit_n: true,
            exit_s: true,
            exit_e: true,
            exit_w: true,
            room_key: room_key
        })
    }
}