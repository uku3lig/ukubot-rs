use once_cell::sync::Lazy;

use crate::bot::misc::{ConfigCommand, EchoCommand, RatioCommand};
use crate::core::SlashCommand;

mod misc;

pub static COMMANDS: Lazy<Vec<&'static dyn SlashCommand>> =
    Lazy::new(|| vec![&RatioCommand, &EchoCommand, &ConfigCommand]);
