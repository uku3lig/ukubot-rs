use std::{collections::HashMap, sync::LazyLock};

use poise::{serenity_prelude::CreateButton, Command};

use crate::{config::Storage, handler::PersistentButton};

mod misc;
mod requests;
mod tag;

pub fn commands() -> Vec<Command<Storage, anyhow::Error>> {
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

pub static BUTTONS: LazyLock<HashMap<String, &'static dyn PersistentButton>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        for button in buttons() {
            if let Some(id) = button_name(&button.create()) {
                map.insert(id, button);
            } else {
                tracing::warn!("button has no custom_id: {:?}", button.create());
            }
        }

        map
    });

// i strongly hate this. but there's not much else i can do sadly
fn button_name(btn: &CreateButton) -> Option<String> {
    let value = serde_json::to_value(btn).ok()?;

    let s = value.as_object()?.get("custom_id")?.as_str()?.to_owned();

    Some(s)
}

#[cfg(test)]
mod test {
    use poise::serenity_prelude as serenity;

    #[test]
    fn test_button_name() {
        const CUSTOM_ID: &str = "some_very_long_custom_id_colon_three";

        let button =
            serenity::CreateButton::new(CUSTOM_ID).label("something completely different!");

        assert_eq!(super::button_name(&button), Some(CUSTOM_ID.to_owned()));
    }
}
