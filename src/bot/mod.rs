use std::collections::HashMap;

use once_cell::sync::Lazy;
use poise::{serenity_prelude::CreateButton, Command};

use crate::handler::PersistentButton;

mod misc;
mod requests;
mod tag;

pub fn commands() -> Vec<Command<(), anyhow::Error>> {
    vec![
        misc::echo(),
        misc::ratio(),
        misc::config(),
        requests::open::open_requests(),
        tag::tag(),
    ]
}

fn buttons() -> Vec<&'static dyn PersistentButton> {
    vec![
        &requests::open::CreateRequestButton,
        &requests::manage::AcceptRequestButton,
        &requests::manage::RejectRequestButton,
        &requests::ticket::FinishRequestButton,
        &requests::ticket::DiscontinueRequestButton,
        &requests::export::ExportButton,
    ]
}

pub static BUTTONS: Lazy<HashMap<String, &'static dyn PersistentButton>> = Lazy::new(|| {
    let mut map = HashMap::new();

    for button in buttons() {
        if let Some(id) = button_name(&button.create()) {
            map.insert(id, button);
        }
    }

    map
});

// i strongly hate this. but there's not much else i can do sadly
fn button_name(btn: &CreateButton) -> Option<String> {
    let value = serde_json::to_value(btn).ok()?;

    let s = value
        .as_array()?
        .first()?
        .as_object()?
        .get("custom_id")?
        .as_str()?
        .to_owned();

    Some(s)
}
