use anyhow::Result;

use crate::{
    jj::{self, types::Bookmark},
    util,
};

/// Attempts to determine which git branch the current commit stack came from
/// and catches it up
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "update")]
pub(crate) struct Args {}

pub(crate) fn main(Args {}: Args) -> Result<()> {
    let dirty = jj::script::count("empty() & @")? == 0;

    if dirty {
        println!("Working set is not clean");
        if util::yn("Run jj new?")? {
            jj::new()?;
        } else {
            println!("Exiting");
            return Ok(());
        }
    }

    let candidates = jj::script::query::<Vec<Bookmark>>(
        "heads(@- & bookmarks()) | root()",
        "self.bookmarks()",
    )?
    .into_iter()
    .flatten()
    .map(|b| b.name)
    .collect::<Vec<_>>();

    let bookmark =
        util::choose("Multiple candidates branches found", &candidates)?;

    let Some(bookmark) = bookmark else {
        println!("There are no existing branches");
        let bookmark = util::prompt("New branch name (Default: main)")?;
        let bookmark = if bookmark.is_empty() {
            "main"
        } else {
            &bookmark
        };
        jj::bookmark::create("@-", bookmark)?;
        return Ok(());
    };

    jj::bookmark::move_(bookmark, "@-")?;
    Ok(())
}
