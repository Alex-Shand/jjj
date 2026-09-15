use anyhow::{Result, bail, ensure};

use crate::jj::{
    self,
    types::{Bookmark, RebaseSource},
};

/// Set a bookmark to keep track of the current active commit then reset to the
/// previous commit
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "stash")]
pub(crate) struct Args {
    #[argh(positional)]
    name_or_cmd: String,
    #[argh(positional)]
    name: Option<String>,
}

pub(crate) fn main(Args { name_or_cmd, name }: Args) -> Result<()> {
    match name_or_cmd.as_str() {
        "list" => {
            ensure!(name.is_none(), "`jj stash list` doesn't take an argument");
            let bookmarks = jj::script::query::<Vec<Bookmark>>(
                "bookmarks()",
                "self.bookmarks()",
            )?
            .into_iter()
            .flatten();
            for bookmark in bookmarks {
                if !bookmark.name.starts_with("stash/") {
                    continue;
                }
                for target in bookmark.target {
                    jj::log(target)?;
                }
            }
        }
        "show" => {
            let Some(name) = name else {
                bail!(
                    "`jj stash show <stash>` requires an argument (use `jj stash list` to see all stashes)"
                )
            };
            jj::show(format!("stash/{name}"))?;
        }
        "pop" => {
            let Some(name) = name else {
                bail!("`jj stash pop <name>` requires an argument");
            };
            jj::rebase(RebaseSource::Source, format!("stash/{name}"), "@")?;
            jj::next()?;
            jj::bookmark::delete(format!("stash/{name}"))?;
        }
        _ => {
            ensure!(
                name.is_none(),
                "`jj stash <stash>` only takes one argument"
            );
            jj::bookmark::create("@", format!("stash/{name_or_cmd}"))?;
            jj::prev()?;
        }
    }
    Ok(())
}
