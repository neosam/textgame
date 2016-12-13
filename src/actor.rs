use std::collections::HashMap;
use item::Item;
use base::Watchable;
use fight::{Attacker, Defender, Fighter};
use fight::DamageRes;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Attribute {
    pub value: u32
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ActorAttributes {
    pub strength: Attribute,
    pub attack: Attribute,
    pub defence: Attribute,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Actor {
    pub keyword: String,
    pub name: String,
    pub health: Container,
    pub attributes: ActorAttributes,
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

impl Attacker for Actor {
    fn get_attack(&self) -> u32 {
        self.attributes.attack.value
    }
}

impl Defender for Actor {
    fn get_defence(&self) -> u32 {
        self.attributes.defence.value
    }
    fn damage(&mut self, damage: u32) -> DamageRes {
        if damage < self.health.value {
            self.health.value -= damage;
            DamageRes::Default(damage)
        } else {
            DamageRes::Dead
        }
    }
}

impl Fighter for Actor {}

