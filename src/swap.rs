//TODO: Swap
// Get ID of @-
// jj rebase -r @ -o @--
// jj rebase -r <ID from step 1> -o @
// jj next --edit

use anyhow::{Result, ensure};

use crate::jj::{self, types::RebaseSource};

//TODO: Swap
// Get ID of @-
// jj rebase -r @ -o @--
// jj rebase -r <ID from step 1> -o @
// jj next --edit

/// Swap @ and @-
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "swap")]
pub(crate) struct Args {}

pub(crate) fn main(Args {}: Args) -> Result<()> {
    let at_minus = jj::script::query::<String>("@-", "self.change_id()")?;
    ensure!(
        at_minus.len() == 1,
        "Expected 1 commit from revest @-, got {}",
        at_minus.len()
    );
    let at_minus = at_minus.into_iter().next().unwrap();

    jj::rebase(RebaseSource::Revision, "@", "@--")?;
    jj::rebase(RebaseSource::Revision, at_minus, "@")?;
    jj::next()?;

    Ok(())
}
