use std::num::ParseIntError;

use anyhow::anyhow;
use serenity::json::Value;

pub trait ParseSnowflake {
    fn parse_snowflake(&self) -> anyhow::Result<u64>;
}

impl ParseSnowflake for Option<Value> {
    fn parse_snowflake(&self) -> anyhow::Result<u64> {
        self.as_ref()
            .ok_or(anyhow!("value is empty"))?
            .as_str()
            .ok_or(anyhow!("value is not a string"))?
            .parse()
            .map_err(|e: ParseIntError| e.into())
    }
}
