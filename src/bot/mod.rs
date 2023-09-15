use crate::bot::misc::{EchoCommand, RatioCommand};
use crate::command::UkubotCommand;
use once_cell::sync::Lazy;

pub mod misc;

pub static COMMANDS: Lazy<Vec<&'static dyn UkubotCommand>> =
    Lazy::new(|| vec![&RatioCommand, &EchoCommand]);
