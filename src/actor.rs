use std::collections::HashMap;
use item::Item;
use base::Watchable;

#[derive(Serialize, Deserialize, Debug)]
pub struct Container {
    pub value: u32,
    pub max: u32
}

/// Value should be at least 1 to avoid immediate death.
impl Default for Container {
    fn default() -> Self {
        Container {
            value: 1,
            max: 1
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Actor {
    pub keyword: String,
    pub name: String,
    pub health: Container,
    pub visible: bool,
    pub display: bool,
    pub items: HashMap<String, Item>
}

impl Actor {
    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.keyword.clone(), item);
    }
}

impl Watchable for Actor {
    fn watch(&self) -> String {
        let mut res = String::with_capacity(1024);
        res.push_str(&self.name);
        res.push('\n');
        if !self.items.is_empty() {
            res.push_str("Items: ");
            res.push_str(
                &self.items.iter()
                    .fold(String::new(), | mut acc, (key, _) | {
                        acc.push_str(key);
                        acc.push(' ');
                        acc
                    })
            );
            res.push('\n');
        }
        res
    }
}