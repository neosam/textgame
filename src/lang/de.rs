use ::lang::Lang;
use base::RoomKey;
use holder::HolderKey;

pub struct LangDe;

impl Lang for LangDe {
    fn welcome(&self) -> String {
        "Willkommen!".to_string()
    }
    fn command_not_found(&self, command: &str) -> String {
        format!("Befehl nicht gefunden: {}", command)
    }
    fn keyword_not_found(&self, keyword: &str) -> String {
        format!("Schlüsselwort nicht gefunden: {}", keyword)
    }

    fn item_prompt(&self) -> String {
        "Item: ".to_string()
    }

    fn room_key_prompt(&self) -> String {
        "Raum Key: {}".to_string()
    }

    fn actor_prompt(&self) -> String {
        "Actor: ".to_string()
    }

    fn room_key_response(&self, room_key: &RoomKey) -> String {
        format!("Raum key ist {}", room_key.get())
    }

    fn room_title_prompt(&self) -> String {
        "Name des Raumes: ".to_string()
    }

    fn exit_name_prompt(&self) -> String {
        "Name des Ausgangs: ".to_string()
    }

    fn exit_not_found_response(&self) -> String {
        "Ausgang nicht gefunden".to_string()
    }

    fn no_damage_response(&self) -> String {
        "Kein Schaden".to_string()
    }

    fn damage_response(&self, damage: u32) -> String {
        format!("Schaden: {}", damage)
    }

    fn dead_response(&self) -> String {
        "Tot".to_string()
    }

    fn multiline_info(&self) -> String {
        "Schreibe beliebig viele Zeilen and beende mit ende".to_string()
    }

    fn description_prompt(&self) -> String {
        "Beschreibung: ".to_string()
    }

    fn default_multiline_term(&self) -> String {
        "ende".to_string()
    }

    fn game_name_info(&self) -> String {
        "Bitte nur Nummern und Buchstaben verwenden".to_string()
    }

    fn game_name_prompt(&self) -> String {
        "Spielname: ".to_string()
    }

    fn invalid_characters_response(&self) -> String {
        "Ungültige Zeichen gefunden".to_string()
    }
}