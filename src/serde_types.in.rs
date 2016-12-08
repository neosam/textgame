#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub value: u32,
    pub max: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    pub name: String,
    pub health: Container,
    pub visible: bool,
    pub display: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exit {
    pub label: String,
    pub room_key: RoomKey
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub title: String,
    pub description: String,
    pub exits: Vec<Exit>,
    pub actors: Vec<ActorKey>
}

key_gen!(RoomKey);
key_gen!(ActorKey);

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub room_ref: RoomKey,
    pub player_ref: ActorKey,
    pub rooms: Holder<Room, RoomKey>,
    pub actors: Holder<Actor, ActorKey>
}
