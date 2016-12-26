use worldgen::context;
use worldgen::direction::Direction;
use base::RoomKey;
use game::Game;
use gameerror::GameError;
use std::result::Result;
use std::error::Error;
use worldgen::context::{GlobalPos, Area, Terrain};
use worldgen::roomgen::RoomGen;
use holder::HolderKey;

pub struct WorldGen {
    context: context::Context,
    null_room_key: RoomKey
}



impl WorldGen {
    /// Return an empty WorldGen.
    pub fn new(area_size: context::Size) -> Self {
        WorldGen {
            context: context::Context::new(area_size),
            null_room_key: RoomKey::new(0)
        }
    }

    pub fn new_and_init(area_size: context::Size, game: &mut Game) -> (Self, GlobalPos) {
        let mut res = WorldGen::new(area_size);
        let middle = ((0, 0), (area_size.0 / 2, area_size.1 / 2));
        res.gen_room(game, middle);
        (res, middle)
    }

    /// Handles world generation actions when the player moves.
    ///
    /// # Algorithm
    /// 1. First it tries to get the global position of the room key.
    ///
    /// 2. Calculate the now inner position from using the direction.
    ///    Later this must check if we enter a new area and have to switch/generate the new
    ///    area.
    ///
    /// 3. Try to load the now dest room, do nothing if it exists.
    ///    Later this can be changed to also perform actions.
    ///
    /// 4. If it doesn't exist, add a new room.
    pub fn do_move(&mut self,
                   game: &mut Game,
                   room_key: RoomKey,
                   direction: Direction) -> Result<(), Box<Error>> {
        // Get current global position
        let (area_pos, room_pos) = self.context.global_pos_map.get(&room_key)
            .ok_or(GameError::GeneralError(
                "Could not get room position in dynamic generation engine".to_string()
            ))?.clone();

        // Get dest room global position
        let dest_room_pos =
            self.context.global_pos_correction((area_pos, direction.add_pos(&room_pos)));

        // Generate the room if it doesn't yet exist
        if !self.context.room_for_pos_exists(&dest_room_pos) {
            self.gen_room(game, dest_room_pos)?;
        }
        Ok(())
    }

    pub fn gen_room(&mut self,
                    game: &mut Game,
                    pos: GlobalPos) -> Result<(), Box<Error>> {
        let (area_pos, inner_pos) = pos;

        // Create area if it doesn't yet exist
        if !self.context.areas.contains_key(&area_pos) {
            self.context.areas.insert(area_pos.clone(), Area::new_random(area_pos.clone()));
        }

        let area: &mut Area = self.context.areas.get_mut(&area_pos)
            .ok_or("Area not found")?;
        if !area.rooms.contains_key(&inner_pos) {
            let room_def = area.gen_room(game, inner_pos, self.null_room_key)?;
            area.rooms.insert(inner_pos, room_def);
        }

        Ok(())
    }

    pub fn room_key_at(&self, pos: GlobalPos) -> Result<RoomKey, Box<Error>> {
        let (area_pos, inner_pos) = pos;
        let area = self.context.areas.get(&area_pos)
            .ok_or("Area not found")?;
        let room_def = area.rooms.get(&inner_pos)
            .ok_or("Room not found")?;
        Ok(room_def.room_key.clone())
    }
}