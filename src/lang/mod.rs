use base::RoomKey;
use holder::HolderKey;
use std::env;

pub trait Lang {
    fn welcome(&self) -> String {
        "Welcome!".to_string()
    }
    fn command_not_found(&self, command: &str) -> String {
        format!("Command not found: {}", command)
    }
    fn keyword_not_found(&self, keyword: &str) -> String {
        format!("Keyword not found: {}", keyword)
    }

    fn item_prompt(&self) -> String {
        "Item: ".to_string()
    }

    fn room_key_prompt(&self) -> String {
        "Room key: ".to_string()
    }

    fn actor_prompt(&self) -> String {
        "Actor: ".to_string()
    }

    fn room_key_response(&self, room_key: &RoomKey) -> String {
        format!("Room key is {}", room_key.get())
    }
    fn exit_label_prompt(&self) -> String {
        "Exit label: ".to_string()
    }

    fn room_title_prompt(&self) -> String {
        "Room title: ".to_string()
    }

    fn exit_name_prompt(&self) -> String {
        "Exit name: ".to_string()
    }

    fn exit_not_found_response(&self) -> String {
        "Exit not found".to_string()
    }

    fn no_damage_response(&self) -> String {
        "No damage".to_string()
    }

    fn damage_response(&self, damage: u32) -> String {
        format!("Damage: {}", damage)
    }

    fn dead_response(&self) -> String {
        "Dead".to_string()
    }

    fn multiline_info(&self) -> String {
        "Write multiple lines and end with end".to_string()
    }

    fn description_prompt(&self) -> String {
        "Description: ".to_string()
    }

    fn default_multiline_term(&self) -> String {
        "end".to_string()
    }

    fn game_name_info(&self) -> String {
        "Write the name.  Only digits and letters are allowed".to_string()
    }

    fn game_name_prompt(&self) -> String {
        "Game name: ".to_string()
    }

    fn invalid_characters_response(&self) -> String {
        "Invalid characters found".to_string()
    }
}

pub mod en;
pub mod de;

use ::lang::en::LangEn;
use ::lang::de::LangDe;

static mut LANG_EN: &'static Lang = &LangEn;
static mut LANG_DE: &'static Lang = &LangDe;

static mut LANG: &'static Lang = &LangEn;

pub fn set_lang(l: &'static Lang) {
    unsafe {
        LANG = l;
    }
}


pub fn set_locale(lang: &str) {
    unsafe {
        if lang.starts_with("de") {
            set_lang(LANG_DE)
        } else if lang.starts_with("en") {
            set_lang(LANG_EN)
        }
    }
}

pub fn init() {
    let lang = env::var("LANG")
        .unwrap_or(String::from("en"));
    set_locale(&lang)
}

pub fn lang() -> &'static Lang {
    unsafe {
        LANG
    }
}
