use once_cell::sync::Lazy;

use crate::bot::misc::{ConfigCommand, EchoCommand, RatioCommand};
use crate::command::UkubotCommand;

mod misc;

pub static COMMANDS: Lazy<Vec<&'static dyn UkubotCommand>> =
    Lazy::new(|| vec![&RatioCommand, &EchoCommand, &ConfigCommand]);
