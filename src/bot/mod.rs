use std::collections::HashMap;

use once_cell::sync::Lazy;
use poise::Command;

use crate::handler::PersistentButton;

mod misc;

pub fn commands() -> Vec<Command<(), anyhow::Error>> {
    vec![misc::echo(), misc::ratio(), misc::config()]
}

fn buttons() -> Vec<&'static dyn PersistentButton> {
    vec![]
}

pub static BUTTONS: Lazy<HashMap<String, &'static dyn PersistentButton>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for button in buttons() {
        if let Some(id) = button.create(&mut Default::default()).0.get("custom_id") {
            map.insert(id.to_string(), button);
        }
    }

    map
});
