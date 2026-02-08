use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Menu {
    Add, View, Update, Remove, Quit,
}

impl Menu {
    pub fn menu_select(selection: &str) -> Option<Menu> {
        let selection = selection.trim().to_lowercase();
        match selection.as_str() {
            "add" | "1" => Some(Menu::Add),
            "view" | "2" => Some(Menu::View),
            "update" | "3" => Some(Menu::Update),
            "remove" | "4" => Some(Menu::Remove),
            "quit" | "q" => Some(Menu::Quit),
            _ => None,
        }
    }
}