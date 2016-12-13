use base::RoomKey;
use std::collections::HashMap;
use item::Item;
use actor::Actor;
use gameerror::GameError;
use std::result::Result;
use std::error::Error;
use fight::Fight;
use fight::DamageRes;

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

    pub fn actor_drop(&mut self, actor_key: &str, item_key: &str) -> Result<(), Box<Error>> {
        let item = {
            let actor = self.actors.get_mut(actor_key)
                .ok_or(GameError::GeneralError("Actor not found".to_string()))?;
            actor.items.remove(item_key)
                .ok_or(GameError::GeneralError("Item not found".to_string()))?
        };
        self.add_item(item);
        Ok(())
    }

    pub fn attack(&mut self, attacker_key: &str, defender_key: &str)
            -> Result<DamageRes, Box<Error>> {
        let res = {
            let attacker: Actor = self.get_actor(attacker_key)
                .ok_or(GameError::GeneralError("Attacker not found".to_string()))?.clone();
            let defender = self.actors.get_mut(defender_key)
                .ok_or(GameError::GeneralError("Defender not found".to_string()))?;
            defender.got_hit(&attacker)
        };
        match res {
            DamageRes::Dead => self.actor_to_corpse(defender_key)?,
            _ => ()
        }
        Ok(res)
    }

    pub fn actor_to_corpse(&mut self, actor_key: &str) -> Result<(), Box<Error>> {
        let actor = self.actors.remove(actor_key)
            .ok_or(GameError::GeneralError("Could not remove actor for dying".to_string()))?;
        let corpse = Item {
            keyword: format!("{}_corpse", actor_key),
            label: format!("{}'s corpse", actor.name),
            description: format!("This is a dead body")
        };
        self.add_item(corpse);
        Ok(())
    }
}