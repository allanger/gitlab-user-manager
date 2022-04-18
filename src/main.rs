mod args;
mod cli;
mod cmd;
mod config;
mod gitlab;
mod output;
mod service;
mod store;
mod types;
use cmd::{
    generate::GenerateCmd, groups, init::InitCmd, search, sync, teams, upgrade, users, Cmd, CmdOld,
};
use output::out_extra::OutExtra;
use std::io::{Error, ErrorKind};
use std::process::exit;
use types::v1::state::State;

const MESSAGE_OF_THE_DAY: &str = "☮️  Fight war, not wars ☮️";
const NEWS: &[&str] = &[
    "IMPORTANT: Rename teams.groups to teams.namespace in your config file, otherwise gum shall not pass",
    "COOL: Now you can generate basic completions for your shell, check the `generate` command out"
];

fn main() {
    OutExtra::welcome_message(MESSAGE_OF_THE_DAY, NEWS);

    match cli::exec(cli::build().get_matches()) {
        Err(err) => {
            OutExtra::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutExtra::sum_success("It was fun, wasn't it?");
        }
    }
}
