use std::env;

pub mod lang;

pub use lang::lang::Lang;

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

pub fn t() -> &'static Lang {
    unsafe {
        LANG
    }
}
