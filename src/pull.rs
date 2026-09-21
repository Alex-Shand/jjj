/// Locate the nearest branch to @, fetch it from remote then rebase the current
/// commit stack back onto the updated branch
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "pull")]
pub(crate) struct Args {}

// jj git fetch
// jj track <branch>
// jj rebase -b @ -o <branch>

pub(crate) fn main(Args {}: Args) {}
