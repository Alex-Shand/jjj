//! jjj
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]
#![allow(clippy::result_large_err)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::missing_errors_doc)]

use anyhow::Result;

mod branch;
mod gc;
mod install;
mod jj;
mod pull;
mod push;
mod stash;
mod swap;
mod switch;
mod update;
mod util;

/// jj helpers
#[allow(missing_copy_implementations)]
#[derive(Debug, argh::FromArgs)]
pub struct Args {
    #[argh(subcommand)]
    command: Command,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Command {
    Install(install::Args),
    Update(update::Args),
    Stash(stash::Args),
    Gc(gc::Args),
    Swap(swap::Args),
    Push(push::Args),
    Pull(pull::Args),
    Branch(branch::Args),
    Switch(switch::Args),
}

#[allow(missing_docs)]
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn main(Args { command }: Args) -> Result<()> {
    match command {
        Command::Install(args) => install::main(args),
        Command::Update(args) => update::main(args),
        Command::Stash(args) => stash::main(args),
        Command::Gc(args) => gc::main(args),
        Command::Swap(args) => swap::main(args),
        Command::Push(args) => push::main(args),
        Command::Pull(args) => pull::main(args),
        Command::Branch(args) => branch::main(args),
        Command::Switch(args) => switch::main(args),
    }
}
