use holder::*;

pub trait Watchable {
    fn watch(&self) -> String;
}

key_gen!(RoomKey);
key_gen!(ActorKey);