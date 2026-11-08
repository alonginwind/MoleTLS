use crate::lang;
use leptos::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Locale {
    Zh,
    En,
}

impl Locale {
    pub fn name(self) -> &'static str {
        match self {
            Locale::Zh => "中文",
            Locale::En => "English",
        }
    }

    pub fn all() -> &'static [Locale] {
        &[Locale::Zh, Locale::En]
    }
}

pub fn t(key: &str, locale: Locale) -> &'static str {
    match locale {
        Locale::Zh => lang::zh::t(key),
        Locale::En => lang::en::t(key),
    }
}

#[derive(Clone, Copy)]
pub struct I18n {
    pub locale: RwSignal<Locale>,
}

impl I18n {
    pub fn new(locale: Locale) -> Self {
        Self {
            locale: create_rw_signal(locale),
        }
    }

    pub fn t(&self, key: &str) -> &'static str {
        t(key, self.locale.get())
    }
}
