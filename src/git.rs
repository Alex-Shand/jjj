use std::{fmt::Display, process::Command};

use anyhow::Result;
use command_ext::CommandExt;

pub(crate) fn checkout(branch: impl AsRef<str>) -> Result<()> {
    Command::new("git")
        .args(["checkout", branch.as_ref()])
        .check_status()?;
    Ok(())
}

pub(crate) fn set_upstream(branch: impl Display) -> Result<()> {
    Command::new("git")
        .arg("branch")
        .arg(format!("--set-upstream-to=origin/{branch}"))
        .check_status()?;
    Ok(())
}
