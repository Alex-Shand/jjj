use anyhow::Result;

use crate::{
    branch,
    jj::{self, types::RebaseSource},
    util,
};

/// Locate the nearest branch to @, fetch it from remote then rebase the current
/// commit stack back onto the updated branch (if the branch isn't local yet use
/// jj switch <branch> to obtain it)
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "pull")]
pub(crate) struct Args {}

// jj git fetch
// jj track <branch>
// jj rebase -b @ -o <branch>

pub(crate) fn main(Args {}: Args) -> Result<()> {
    let branches = branch::get_nearest_branches()?;
    let branch = util::choose("Multiple possible branches to pull", &branches)?;
    let Some(branch) = branch else {
        println!(
            "No local branches. Use `jj switch <branch>` to obtain a remote branch or `jj update` to init a main branch"
        );
        return Ok(());
    };
    jj::git::fetch()?;
    jj::rebase(RebaseSource::Branch, "@", branch)?;
    Ok(())
}
