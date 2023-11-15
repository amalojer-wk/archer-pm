mod cli;
mod modifier;
mod util;

use clap::Parser;

use cli::{CLIArgs, Command};
use modifier::execute_modifier_op;

fn main() {
    let args = CLIArgs::parse();

    match args.command {
        Command::Manager => eprintln!("Error: Manager is not enabled."),
        Command::Modifier { operation } => execute_modifier_op(operation),
    }
}
