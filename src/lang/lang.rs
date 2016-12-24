use base::RoomKey;
use holder::HolderKey;

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

    fn items_prompt(&self) -> String {
        "Items: ".to_string()
    }

    fn room_key_prompt(&self) -> String {
        "Room key: ".to_string()
    }

    fn exits_prompt(&self) -> String {
        "Exits: ".to_string()
    }

    fn actor_prompt(&self) -> String {
        "Actor: ".to_string()
    }

    fn actors_prompt(&self) -> String {
        "Actors: ".to_string()
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

    fn player_id(&self) -> String {
        "you".to_string()
    }

    fn actor_in_room_nof_found_response(&self) -> String {
        "Actor in room not found".to_string()
    }

    fn actor_key_not_found_response(&self) -> String {
        "Actor key nof found".to_string()
    }

    fn item_key_not_found_response(&self) -> String {
        "Item key not found".to_string()
    }

    fn actor_not_found_response(&self) -> String {
        "Actor not found".to_string()
    }

    fn item_not_found_response(&self) -> String {
        "Item nof found".to_string()
    }

    fn attacker_not_found_response(&self) -> String {
        "Attacker nof found response".to_string()
    }

    fn defender_nof_found_response(&self) -> String {
        "Defender nof found response".to_string()
    }

    fn cannot_remove_actor_die_error(&self) -> String {
        "Cannot remove actor to die".to_string()
    }

    fn to_corpse_keyword(&self, keyword: &str) -> String {
        format!("{}_corpse", keyword)
    }

    fn to_corpse_label(&self, name: &str) -> String {
        format!("{}'s corpse", name)
    }

    fn dead_body_description(&self) -> String {
        "This is a dead body".to_string()
    }

    fn command_look(&self) -> String {
        "look".to_string()
    }

    fn command_lookitem(&self) -> String {
        "lookitem".to_string()
    }

    fn command_lookactor(&self) -> String {
        "lookactor".to_string()
    }

    fn command_take(&self) -> String {
        "take".to_string()
    }

    fn command_drop(&self) -> String {
        "drop".to_string()
    }

    fn command_attack(&self) -> String {
        "attack".to_string()
    }

    fn command_quit(&self) -> String {
        "quit".to_string()
    }

    fn command_go(&self) -> String {
        "go".to_string()
    }

    fn command_save(&self) -> String {
        "save".to_string()
    }

    fn command_load(&self) -> String {
        "load".to_string()
    }
}