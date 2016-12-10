use holder::*;
use actor::*;
use room::*;
use base::{RoomKey, Watchable};
use gameerror::*;
use std::result::Result;
use std::error::Error;



#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub room_ref: RoomKey,
    pub player_ref: String,
    pub rooms: Holder<Room, RoomKey>,
}

impl Game {
    pub fn new() -> Self {
        let mut room_holder: Holder<Room, RoomKey> = Holder::default();
        let room_ref = room_holder.add(Room::default());
        Game {
            room_ref: room_ref,
            player_ref: "you".to_string(),
            rooms: room_holder,
        }
    }

    pub fn add_room(&mut self, room: Room) ->  RoomKey {
        self.rooms.add(room)
    }

    pub fn room_ref(&self, id: RoomKey) -> &Room {
        self.rooms.get(id)
    }
    pub fn room_mut(&mut self, id: RoomKey) -> &mut Room {
        self.rooms.get_mut(id)
    }

    pub fn player_room(&self) -> &Room {
        self.room_ref(self.room_ref)
    }
    pub fn player_room_mut(&mut self) -> &mut Room {
        let room_ref = self.room_ref;
        self.room_mut(room_ref)
    }

    pub fn warp_actor(&mut self, actor_key: String, from_room_key: RoomKey, to_room_key: RoomKey)
            -> Result<(), Box<Error>> {
        let actor: Actor = {
            let mut from_room = self.room_mut(from_room_key);
            from_room.actors.remove(&actor_key)
                .ok_or(GameError::GeneralError("Actor in from room not found".to_string()))?
        };
        let mut to_room = self.room_mut(to_room_key);
        to_room.add_actor(actor);
        Ok(())
    }
}

impl Watchable for Room {
    fn watch(&self) -> String {
        let mut res = String::with_capacity(2048);
        res.push_str(&self.title);
        res.push_str("\n");
        res.push_str(&self.description);
        res.push_str("\n");
        if !self.items.is_empty() {
            res.push_str("Items: ");
            res.push_str(
                &self.items.iter().fold(String::new(), |mut acc, (keyword, _)| {
                    acc.push_str(keyword);
                    acc
                })
            );
            res.push('\n');
        }
        if !self.actors.is_empty() {
            res.push_str("Actors: ");
            res.push_str(
                &self.actors.iter().fold(String::new(), |mut acc, (key, _)| {
                    acc.push_str(key);
                    acc
                })
            );
            res.push('\n');
        }
        if !self.exits.is_empty() {
            res.push_str("Exits: ");
            res.push_str(
                &self.exits.iter().fold(String::new(), |mut acc, (key, _) | {
                    acc.push_str(key);
                    acc.push_str(" ");
                    acc
                })
            );
            res.push('\n');
        }
        res
    }
}