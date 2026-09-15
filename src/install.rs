use std::fs;

use anyhow::{Context, Result, anyhow};
use toml_edit::DocumentMut;

use crate::jj;

static ALIASES: &str = include_str!("install/aliases.toml");

/// Install jjj aliases
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "install")]
pub(crate) struct Args {}

pub(crate) fn main(Args {}: Args) -> Result<()> {
    let aliases = ALIASES
        .parse::<DocumentMut>()
        .expect("Embedded aliases are invalid");
    let aliases = aliases.as_table();
    let config = jj::config_path()?;
    let mut contents = fs::read_to_string(&config)
        .with_context(|| anyhow!("Failed to read {config}"))?
        .parse::<DocumentMut>()
        .with_context(|| anyhow!("Failed to parse {config}"))?;
    if let Some(table) =
        contents.get_mut("aliases").and_then(|i| i.as_table_mut())
    {
        for (key, item) in aliases {
            let _ = table.insert(key, item.to_owned());
        }
    } else {
        let _ = contents.insert("aliases", aliases.to_owned().into());
    }
    fs::write(&config, contents.to_string())
        .with_context(|| anyhow!("Failed to write to {config}"))?;
    Ok(())
}
