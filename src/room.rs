use base::RoomKey;
use std::collections::HashMap;
use item::Item;
use actor::Actor;
use gameerror::GameError;
use std::result::Result;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Exit {
    pub label: String,
    pub room_key: RoomKey
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Room {
    pub title: String,
    pub description: String,
    pub exits: HashMap<String, Exit>,
    pub actors: HashMap<String, Actor>,
    pub items: HashMap<String, Item>
}

impl Room {
    pub fn with_title(title: String) -> Self {
        let mut room = Room::default();
        room.title = title;
        room
    }

    pub fn add_exit(&mut self, exit: Exit) {
        self.exits.insert(exit.label.clone(), exit);
    }

    pub fn get_exit(&self, direction: String) -> Option<&Exit> {
        self.exits.get(&direction)
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.keyword.clone(), item);
    }
    pub fn get_item(&self, keyword: &str) -> Option<&Item> {
        self.items.get(keyword)
    }

    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.insert(actor.keyword.clone(), actor);
    }
    pub fn get_actor(&self, keyword: &str) -> Option<&Actor> {
        self.actors.get(keyword)
    }

    pub fn actor_take(&mut self, actor_key: &str, item_key: &str) -> Result<(), Box<Error>> {
        if !self.actors.contains_key(actor_key) {
            return Err(Box::new(GameError::GeneralError("Actor key not found".to_string())));
        }
        if !self.items.contains_key(item_key) {
            return Err(Box::new(GameError::GeneralError("Item key not found".to_string())));
        }
        let item = self.items.remove(item_key).unwrap();
        self.actors.get_mut(actor_key).unwrap().add_item(item);
        Ok(())
    }
}