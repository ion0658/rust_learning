use anyhow::Result;
use dotenvy::dotenv;
use std::env;

/// Show All environment Values
fn main() -> Result<()> {
    dotenv()?;
    env::vars()
        .into_iter()
        .for_each(|(key, val)| println!("{key}: {value}", key = key, value = val));
    Ok(())
}
