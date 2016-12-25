use std::collections::HashMap;
use base::RoomKey;
use worldgen::city::City;
use game::Game;
use worldgen::roomgen::RoomGen;
use holder::HolderKey;


pub type Pos = (u32, u32);
/// Holds (area position, inner position).
pub type GlobalPos = (Pos, Pos);
pub type Size = (u32, u32);

pub enum Terrain {
    City(City),
/*    Grassland,
    Greenland,
    Mountains*/
}

pub struct RoomDef {
    pub room_key: RoomKey,
    pub exit_n: bool,
    pub exit_s: bool,
    pub exit_w: bool,
    pub exit_e: bool
}

pub struct Area {
    pub pos: Pos,
    pub terrain: Terrain,
    pub rooms: HashMap<Pos, Option<RoomDef>>
}

impl Area {
    pub fn gen_room(&mut self, game: &mut Game, inner_pos: Pos) -> RoomDef {
        unimplemented!();
    }
}

impl Area {
    pub fn new_random(pos: Pos) -> Self {
        Area {
            pos: pos,
            terrain: Terrain::City(City),
            rooms: HashMap::new()
        }
    }
}

pub struct Context {
    pub area_size: Size,
    pub areas: HashMap<Pos, Area>,
    pub global_pos_map: HashMap<RoomKey, GlobalPos>
}

impl Context {
    pub fn new(area_size: Size) -> Self {
        Context {
            area_size: area_size,
            areas: HashMap::new(),
            global_pos_map: HashMap::new()
        }
    }

    pub fn room_for_pos_exists(&self, &(area_pos, inner_pos): &GlobalPos) -> bool {
        let area = match self.areas.get(&area_pos) {
            Some(area) => area,
            None => return false
        };
        area.rooms.contains_key(&inner_pos)
    }

    pub fn global_pos_correction(&mut self,
                                 ((area_x, area_y), (x, y)): GlobalPos) -> GlobalPos {
        let (width, height) = self.area_size;
        (
            (area_x + x / width, area_y + y / height),
            (x % width, y % height)
        )
    }
}