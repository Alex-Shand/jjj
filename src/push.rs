use std::collections::HashSet;

use anyhow::Result;

use crate::{branch, git, jj, util};

/// With no argument:
///   Find the nearest branch:
///     If it is at @ push it
///     If it is at @- and @ is empty push it
///     If it is at @- and @ is not empty warn but push anyway
///     If it is anywhere else refuse to push and suggest running `jj update`
/// With argument:
///   If the named branch doesn't already exist create it at @ (if non-empty) or
///   @- (if @ is empty) then push
///   If the named branch does exist but isn't a candidate for the nearest
///   branch bail
///   If the named branch is a candidate for the nearest branch proceed as with
///   no arguments
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "push")]
pub(crate) struct Args {
    #[argh(positional)]
    name: Option<String>,
}

pub(crate) fn main(Args { name }: Args) -> Result<()> {
    if let Some(name) = name {
        let bookmarks = jj::bookmark::list()?
            .into_iter()
            .map(|b| b.name)
            .collect::<HashSet<String>>();
        if bookmarks.contains(&name) {
            let nearest_branches = branch::get_nearest_branches()?;
            if nearest_branches.contains(&name) {
                try_push_branch(&name)?;
            } else {
                println!(
                    "Branch {name} exists but isn't checked out. Try `jj switch {name}` or `jj pull`"
                );
                return Ok(());
            }
        } else {
            push_new_bookmark(&name)?;
        }
    } else {
        let nearest_branches = branch::get_nearest_branches()?;
        let branch = util::choose(
            "Multiple possible branches to push",
            &nearest_branches,
        )?;
        let Some(branch) = branch else {
            println!(
                "No available branches to push. Use `jj push <branch name>` to create a new branch"
            );
            return Ok(());
        };
        try_push_branch(branch)?;
    }

    Ok(())
}

fn push_new_bookmark(name: &str) -> Result<()> {
    let target = if jj::script::count("@ & empty()")? == 0 {
        "@"
    } else {
        "@-"
    };
    jj::git::push::named(name, target)?;
    git::set_upstream(name)?;
    Ok(())
}

fn try_push_branch(name: &str) -> Result<()> {
    let can_push;
    if jj::script::count(format!("{name} & @"))? == 1 {
        can_push = true;
    } else if jj::script::count(format!("{name} & @-"))? == 1 {
        if jj::script::count("@ & empty()")? == 0 {
            can_push =
                util::yn(format_args!("Working set is dirty. Push anyway?"))?;
        } else {
            can_push = true;
        }
    } else {
        println!(
            "Branch {name} is behind the commit stack. Run `jj update` to update it"
        );
        can_push = false;
    }

    if can_push {
        jj::git::push::bookmark(name)?;
        git::set_upstream(name)?;
    }

    Ok(())
}
