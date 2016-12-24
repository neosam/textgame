use base::RoomKey;
use std::collections::HashMap;
use item::Item;
use actor::Actor;
use gameerror::GameError;
use std::result::Result;
use std::error::Error;
use fight::{Attacker, Defender};
use fight::DamageRes;
use lang::t;

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
            return Err(Box::new(GameError::GeneralError(t().actor_key_not_found_response())))
        }
        if !self.items.contains_key(item_key) {
            return Err(Box::new(GameError::GeneralError(t().item_key_not_found_response())));
        }
        let item = self.items.remove(item_key).unwrap();
        self.actors.get_mut(actor_key).unwrap().add_item(item);
        Ok(())
    }

    pub fn actor_drop(&mut self, actor_key: &str, item_key: &str) -> Result<(), Box<Error>> {
        let item = {
            let actor = self.actors.get_mut(actor_key)
                .ok_or(t().actor_not_found_response())?;
            actor.items.remove(item_key)
                .ok_or(t().item_not_found_response())?
        };
        self.add_item(item);
        Ok(())
    }

    pub fn attack(&mut self, attacker_key: &str, defender_key: &str)
            -> Result<DamageRes, Box<Error>> {
        let res = {
            let attacker = self.get_actor(attacker_key)
                .ok_or(t().attacker_not_found_response())?.to_attacker();
            let defender = self.actors.get_mut(defender_key)
                .ok_or(t().defender_nof_found_response())?;
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
            .ok_or(t().cannot_remove_actor_die_error())?;
        let corpse = Item {
            keyword: t().to_corpse_keyword(actor_key),
            label: t().to_corpse_label(&actor.name),
            description: t().dead_body_description()
        };
        self.add_item(corpse);
        Ok(())
    }
}