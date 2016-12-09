use holder::*;
use actor::*;
use room::*;
use base::{RoomKey, ActorKey, Watchable};



#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub room_ref: RoomKey,
    pub player_ref: ActorKey,
    pub rooms: Holder<Room, RoomKey>,
    pub actors: Holder<Actor, ActorKey>
}

impl Game {
    pub fn new() -> Self {
        let mut room_holder: Holder<Room, RoomKey> = Holder::new();
        let mut actor_holder: Holder<Actor, ActorKey> = Holder::new();
        let room_ref = room_holder.add(Room::new());
        let actor_ref = actor_holder.add(Actor::new());
        Game {
            room_ref: room_ref,
            player_ref: actor_ref,
            rooms: room_holder,
            actors: actor_holder
        }
    }

    pub fn add_room(&mut self, room: Room) ->  RoomKey {
        self.rooms.add(room)
    }

    pub fn room_ref<'a>(&'a self, id: RoomKey) -> RoomGame<'a> {
        RoomGame {
            room: self.rooms.get(id),
            game: self
        }
    }
    pub fn room_mut<'a>(&'a mut self, id: RoomKey) -> RoomGameMut<'a> {
        RoomGameMut {
            room_key: id,
            game: self
        }
    }

    pub fn player_room<'a>(&'a self) -> RoomGame<'a> {
        self.room_ref(self.room_ref)
    }
    pub fn player_room_mut<'a>(&'a mut self) -> RoomGameMut<'a> {
        let room_ref = self.room_ref;
        self.room_mut(room_ref)
    }

    pub fn remove_actor_all_rooms(&mut self, key: ActorKey) {
        for room in self.rooms.items.iter_mut() {
            let new_actors = room.actors.iter()
                .map(|actor_ref| *actor_ref)
                .filter(|actor_ref| *actor_ref != key)
                .collect();
            room.actors = new_actors;
        }
    }
    pub fn warp_actor(&mut self, actor_key: ActorKey, room_key: RoomKey) {
        self.remove_actor_all_rooms(actor_key);
        self.rooms.get_mut(room_key).actors.push(actor_key);
    }
}

pub struct RoomGame<'a> {
    pub room: &'a Room,
    pub game: &'a Game
}
impl<'a> RoomGame<'a> {
    pub fn room_exits(&'a self) -> impl Iterator<Item=&'a Exit> + 'a {
        Box::new(self.room.exits.iter())
    }
    pub fn actors(&'a self) -> impl Iterator<Item=(ActorKey, &'a Actor)> + 'a {
        self.room.actors
            .iter()
            .map(move | actor_key | (*actor_key, self.game.actors.get(*actor_key)))
    }
}

pub struct RoomGameMut<'a> {
    pub room_key: RoomKey,
    pub game: &'a mut Game
}
impl<'a> RoomGameMut<'a> {
    pub fn room(&mut self) -> &mut Room {
        self.game.rooms.get_mut(self.room_key)
    }

    pub fn as_ref<'b>(&'b self) -> RoomGame<'b> {
        RoomGame::<'b> {
            room: self.game.rooms.get(self.room_key),
            game: self.game
        }
    }

    pub fn add_actor(&mut self, a: Actor) {
        let actor_key = self.game.actors.add(a);
        self.room().actors.push(actor_key)
    }
}

impl<'a> Watchable for RoomGame<'a> {
    fn watch(&self) -> String {
        let mut res = String::with_capacity(2048);
        res.push_str(&self.room.title);
        res.push_str("\n");
        res.push_str(&self.room.description);
        res.push_str("\n");
        res.push_str(
            &self.actors().fold(String::new(), | mut acc, (_, actor) | {
                acc.push_str(&actor.name.to_string()); acc
            })
        );
        res.push_str("\n");
        res.push_str(
            &self.room.exits.iter().fold(String::new(), | mut acc, exit | {
                acc.push_str(&exit.label);
                acc.push_str(" ");
                acc
            })
        );
        res
    }
}