use std::collections::HashMap;

use once_cell::sync::Lazy;
use poise::Command;

use crate::handler::PersistentButton;

mod misc;
mod requests;

pub fn commands() -> Vec<Command<(), anyhow::Error>> {
    vec![
        misc::echo(),
        misc::ratio(),
        misc::config(),
        requests::open::open_requests(),
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
        if let Some(id) = button.create(&mut Default::default()).0.get("custom_id") {
            map.insert(id.as_str().unwrap().into(), button);
        }
    }

    map
});
