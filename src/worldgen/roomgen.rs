use worldgen::context;
use room::Room;
use std::result::Result;
use std::error::Error;
use base::RoomKey;
use game::Game;

pub trait RoomGen {
    fn gen_room(&mut self,
                game: &mut Game,
                area: &context::Area,
                pos: &context::Pos,
                null_room_key: RoomKey) -> Result<context::RoomDef, Box<Error>>;
}