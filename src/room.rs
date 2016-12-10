use base::RoomKey;
use base::ActorKey;
use std::collections::HashMap;
use item::Item;

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
    pub actors: Vec<ActorKey>,
    pub items: HashMap<String, Item>
}

impl Room {
    pub fn new() -> Room {
        Room {
            title: "".to_string(),
            description: "".to_string(),
            exits: Vec::new(),
            actors: Vec::new(),
            items: HashMap::new()
        }
    }
    pub fn with_title(title: String) -> Self {
        Room {
            title: title,
            description: "".to_string(),
            exits: Vec::new(),
            actors: Vec::new(),
            items: HashMap::new()
        }
    }

    pub fn add_exit(&mut self, exit: Exit) {
        self.exits.push(exit);
    }

    pub fn get_exit(&self, direction: String) -> Option<&Exit> {
        for exit in self.exits.iter() {
            if exit.label == direction {
                return Some(exit)
            }
        }
        None
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.keyword.clone(), item);
    }
}