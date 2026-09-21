use anyhow::Result;

use crate::jj::{self, types::Bookmark};

/// Branch commands
///
/// With no other arguments print the nearest branch(es)
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "branch")]
pub(crate) struct Args {
    /// print all local branches (use git branch -a for known remotes too)
    #[argh(switch)]
    all: bool,
    /// delete the named branch (Note: Does not delete the associated commits,
    /// use jj gc if that is required)
    #[argh(switch)]
    delete: bool,
    /// if passed create a new branch, unless delete is also passed in which
    /// case delete the named branch
    #[argh(positional)]
    name: Option<String>,
}

pub(crate) fn main(Args { all, delete, name }: Args) -> Result<()> {
    if all {
        for branch in branches("bookmarks()")? {
            if branch.starts_with("stash/") {
                continue;
            }
            println!("{branch}");
        }
    }

    if let Some(name) = name {
        if delete {
            jj::bookmark::delete(name)?;
            return Ok(());
        }

        // Create the branch at the first non-empty commit behind @. This will
        // probably be @ or @- in a normal repo
        jj::bookmark::create("heads(::@ ~ empty())", &name)?;
        // If the new branch now points at @ run jj new
        if jj::script::count(format!("{name} & @"))? == 1 {
            jj::new()?;
        }

        return Ok(());
    }

    for branch in get_nearest_branches()? {
        println!("{branch} ~ {}", calculate_offset(&branch)?);
    }
    Ok(())
}

pub(crate) fn get_nearest_branches() -> Result<Vec<String>> {
    branches("heads(::@ & bookmarks()) | root()")
}

fn branches(revset: impl AsRef<str>) -> Result<Vec<String>> {
    Ok(
        jj::script::query::<Vec<Bookmark>>(revset, "self.bookmarks()")?
            .into_iter()
            .flatten()
            .map(|b| b.name)
            .collect(),
    )
}

fn calculate_offset(branch: &str) -> Result<String> {
    let offset = jj::script::count(format!("{branch}..@"))?;
    if offset == 0 {
        return Ok(String::from("points at working commit"));
    }
    if offset == 1 {
        return Ok(String::from("Up-to-date"));
    }
    if offset == 2 {
        return Ok(String::from("1 commit behind"));
    }
    let offset = offset - 1;
    Ok(format!("{offset} commits behind"))
}
