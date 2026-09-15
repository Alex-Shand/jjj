use anyhow::Result;

use crate::jj;

/// Garbage collect any commits that git would consider detached
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "gc")]
pub(crate) struct Args {
    #[argh(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum SubCommand {
    Empty(Empty),
}

/// Only GC empty commits
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "empty")]
struct Empty {}

pub(crate) fn main(Args { subcommand }: Args) -> Result<()> {
    if let Some(SubCommand::Empty(Empty {})) = subcommand {
        // empty() -> Empty commits
        // empty() ~ merges() -> Empty commits which aren't merge commits
        // description(exact:"") -> Commits with an empty description
        // heads(all()) -> Leaf commits
        // (empty() ~ merges()) & description(exact:"") & heads(all()) -> Empty
        // commits which aren't merges and have empty descriptions and are
        // leaves (merge check is probably redundant here but whatever)
        // (empty() ~ merges() ~ heads(all())) -> Empty, non-merge commits which
        // aren't leaves (selects embedded empty commits even if they have a
        // description)
        // ~ root() -> Remove the root commit since we can't delete it
        let empty_query = "((empty() ~ merges()) & description(exact:\"\") & heads(all()) | (empty() ~ merges() ~ heads(all()))) ~ root()";
        jj::abandon(empty_query)?;
        return Ok(());
    }
    // Commits which are not ancestors of bookmarks, should correspond to things
    // git doesn't think is on a branch I think.
    let branch_query = "bookmarks()..";

    // If root is matched by the query then there are no branches, refuse to
    // delete anything
    if jj::script::count(format!("({branch_query}) & root()"))? == 1 {
        println!(
            "There are no git branches in this repository. Refusing to GC"
        );
        return Ok(());
    }
    jj::abandon(branch_query)?;
    Ok(())
}
