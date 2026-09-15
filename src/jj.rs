use std::process::Command;

use anyhow::Result;
use command_ext::CommandExt as _;

pub(crate) mod types {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub(crate) struct Bookmark {
        pub(crate) name: String,
        pub(crate) target: Vec<String>,
    }

    #[derive(Debug, Copy, Clone)]
    pub(crate) enum RebaseSource {
        Revision,
        Source,
    }
}

pub(crate) fn new() -> Result<()> {
    Command::new("jj").arg("new").check_status()?;
    Ok(())
}

pub(crate) fn log(revset: impl AsRef<str>) -> Result<()> {
    Command::new("jj")
        .args(["log", "-r", revset.as_ref()])
        .check_status()?;
    Ok(())
}

pub(crate) fn show(revset: impl AsRef<str>) -> Result<()> {
    Command::new("jj")
        .args(["show", "-r", revset.as_ref()])
        .check_status()?;
    Ok(())
}

pub(crate) fn prev() -> Result<()> {
    Command::new("jj").args(["prev", "--edit"]).check_status()?;
    Ok(())
}

pub(crate) fn next() -> Result<()> {
    Command::new("jj").args(["next", "--edit"]).check_status()?;
    Ok(())
}

pub(crate) fn rebase(
    source: types::RebaseSource,
    revset: impl AsRef<str>,
    target: impl AsRef<str>,
) -> Result<()> {
    let source = match source {
        types::RebaseSource::Revision => "-r",
        types::RebaseSource::Source => "-s",
    };
    Command::new("jj")
        .args(["rebase", source, revset.as_ref(), "-o", target.as_ref()])
        .check_status()?;
    Ok(())
}

pub(crate) mod config {
    use std::process::Command;

    use anyhow::Result;
    use camino::Utf8PathBuf as PathBuf;
    use command_ext::CommandExt as _;

    pub(crate) fn path() -> Result<PathBuf> {
        Ok(PathBuf::from(
            Command::new("jj")
                .args(["config", "path", "--user"])
                .check_output()?
                .trim(),
        ))
    }
}

pub(crate) mod bookmark {
    use std::process::Command;

    use anyhow::Result;
    use command_ext::CommandExt as _;

    pub(crate) fn create(
        revset: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<()> {
        Command::new("jj")
            .args(["bookmark", "create", "-r", revset.as_ref(), name.as_ref()])
            .check_status()?;
        Ok(())
    }

    pub(crate) fn move_(
        name: impl AsRef<str>,
        target_revset: impl AsRef<str>,
    ) -> Result<()> {
        Command::new("jj")
            .args([
                "bookmark",
                "move",
                "--from",
                name.as_ref(),
                "--to",
                target_revset.as_ref(),
            ])
            .check_status()?;
        Ok(())
    }

    pub(crate) fn delete(name: impl AsRef<str>) -> Result<()> {
        Command::new("jj")
            .args(["bookmark", "delete", name.as_ref()])
            .check_status()?;
        Ok(())
    }
}

pub(crate) mod script {
    use std::{fmt::Display, process::Command};

    use anyhow::Result;
    use command_ext::CommandExt as _;
    use serde::Deserialize;

    pub(crate) fn count(revset: impl AsRef<str>) -> Result<usize> {
        Ok(Command::new("jj")
            .args(["log", "--count", "-r", revset.as_ref()])
            .check_output()?
            .trim()
            .parse()?)
    }
    pub(crate) fn query<T: for<'a> Deserialize<'a>>(
        revset: impl AsRef<str>,
        template: impl Display,
    ) -> Result<Vec<T>> {
        let result = Command::new("jj")
            .args(["log", "--no-graph", "-r", revset.as_ref(), "-T"])
            .arg(format!("json({template}) ++ \"\\n\""))
            .check_output()?;
        result
            .lines()
            .map(serde_json::from_str)
            .map(|r| r.map_err(Into::into))
            .collect()
    }
}
