use std::{
    fmt::Display,
    io::{self, Write as _},
};

use anyhow::Result;

pub(crate) fn prompt(prompt: impl Display) -> Result<String> {
    write!(io::stdout(), "{prompt}: ")?;
    io::stdout().flush()?;
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf)?;
    let _ = buf.pop();
    Ok(buf)
}

pub(crate) fn yn(prompt: impl Display) -> Result<bool> {
    loop {
        let buf = self::prompt(format_args!("{prompt} [y/n]"))?;
        match buf.as_str() {
            "y" | "Y" => return Ok(true),
            "n" | "N" => return Ok(false),
            _ => (),
        }
    }
}

pub(crate) fn choose<T: Display>(
    prompt: impl Display,
    options: &[T],
) -> Result<Option<&T>> {
    if options.is_empty() {
        return Ok(None);
    }
    if options.len() == 1 {
        return Ok(options.first());
    }
    loop {
        let msg = format!(
            "{prompt}:\n{}",
            options
                .iter()
                .enumerate()
                .map(|(i, o)| format!("{i}: {o}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        println!("{msg}");
        let buf = self::prompt("Select one")?;
        let idx = buf.parse::<usize>();
        let Ok(idx) = idx else {
            continue;
        };
        let Some(item) = options.get(idx) else {
            continue;
        };
        return Ok(Some(item));
    }
}
