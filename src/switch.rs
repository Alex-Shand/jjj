use std::collections::HashSet;

use anyhow::Result;

use crate::{jj, util};

/// Switch to the specified branch
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "switch")]
pub(crate) struct Args {
    #[argh(positional)]
    name: String,
}

// TODO: If the branch doesn't exist switch should try to fetch it from remote
pub(crate) fn main(Args { name }: Args) -> Result<()> {
    // If @ is non-empty we check if @ or @- are branches. If @ is empty we
    // still check if @- is a branch. We allow the switch to go ahead even if
    // there isn't a branch at @ or @-, the commits aren't going anywhere
    // (unless somebody runs jj gc) but it will be harder to get back here
    // without a branch
    if jj::script::count("@ & empty()")? == 0
        && jj::script::count("(@ & bookmarks()) | (@- & bookmarks())")? == 0
        && !util::yn("There is no branch located at @ or @-. Switch anyway?")?
    {
        println!("Exiting");
        return Ok(());
    }
    if jj::script::count("@- & bookmarks()")? == 0
        && !util::yn("There is no branch located at @-. Switch anyway?")?
    {
        println!("Exiting");
        return Ok(());
    }

    let bookmarks = jj::bookmark::list()?
        .into_iter()
        .map(|b| b.name)
        .collect::<HashSet<String>>();

    if !bookmarks.contains(&name) {
        println!("No local branch called {name}. Checking remote");
        jj::git::fetch()?;
        jj::bookmark::track(&name)?;
        println!("That might have worked, switching anyway");
    }

    switch_to(name)?;

    // Now @ is the specified branch, so we're editing its top commit which is
    // probably the wrong thing to do. If @+ exists assume it is a working set
    // attached to this branch and edit that instead (based on the checks above
    // we shouldn't typically see anything more complicated)
    if jj::script::count("@+")? != 0 {
        jj::next()?;
        return Ok(());
    }

    // Otherwise run jj new to get a new working set on top of this branch
    jj::new()?;
    Ok(())
}

pub(crate) fn switch_to(branch: impl AsRef<str>) -> Result<()> {
    jj::edit(branch)?;
    Ok(())
}
