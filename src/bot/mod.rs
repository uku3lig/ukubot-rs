use once_cell::sync::Lazy;

use crate::bot::misc::{ConfigCommand, EchoCommand, RatioCommand};
use crate::core::{PersistentButton, SlashCommand};

mod misc;

pub static COMMANDS: Lazy<Vec<&'static dyn SlashCommand>> =
    Lazy::new(|| vec![&RatioCommand, &EchoCommand, &ConfigCommand]);

pub static BUTTONS: Lazy<Vec<&'static dyn PersistentButton>> = Lazy::new(|| vec![]);
