use std::collections::HashMap;
use base::RoomKey;
use worldgen::city::City;
use game::Game;
use worldgen::roomgen::RoomGen;
use holder::HolderKey;
use std::error::Error;
use std::result::Result;


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
    pub rooms: HashMap<Pos, RoomDef>
}

impl Area {
    pub fn gen_room(&mut self,
                    game: &mut Game,
                    inner_pos: Pos,
                    null_room: RoomKey) -> Result<RoomDef, Box<Error>> {
        match self.terrain {
            Terrain::City(ref mut city) => city.gen_room(game, &inner_pos, null_room)
        }
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
        let init_area = Area::new_random((0, 0));
        let mut res = Context {
            area_size: area_size,
            areas: HashMap::new(),
            global_pos_map: HashMap::new()
        };
        res.insert_area(init_area);
        res
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

    pub fn insert_area(&mut self, area: Area) {
        self.areas.insert(area.pos.clone(), area);
    }
}