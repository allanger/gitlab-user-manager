mod args;
mod cmd;
mod gitlab;
mod output;
mod service;
mod types;
mod cli;
mod store;

use cmd::{
    init::InitCmd,
    generate::GenerateCmd,
    upgrade,
    groups,
    teams,
    sync,
    users,
    search,
    Cmd, CmdOld,
};
use output::{out_extra::OutExtra, out_message::OutMessage};
use types::v1::state::State;
>>>>>>> 21cc23b (wip)
use std::io::{Error, ErrorKind};
use std::process::exit;

const MESSAGE_OF_THE_DAY: &str = "☮️  Fight war, not wars ☮️";
const NEWS: &[&str] = &[
    "IMPORTANT: Rename teams.groups to teams.namespace in your config file, otherwise gum shall not pass",
    "COOL: Now you can generate basic completions for your shell, check the `generate` command out"
];


fn main() {
    State::get("s3://bucket_name/filepath".to_string());
    State::get("./gum-state.json".to_string());
    State::get("/tmp/gum-state.json".to_string());
    State::get(r#"{"6006629":{"entity":"User","projects":{},"namespaces":{"7818000":"Developer"}}}"#.to_string());
    exit(1);
    OutExtra::welcome_message(MESSAGE_OF_THE_DAY, NEWS);
    let matches = cli::build().get_matches();

    let result: Result<(), Error>;

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            result = match InitCmd::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("generate", sub_matches)) => {
            result = match GenerateCmd::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("sync", sub_matches)) => {
            result = match sync::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("users", sub_matches)) => {
            result = match users::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("groups", sub_matches)) => {
            result = match groups::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        Some(("teams", sub_matches)) => {
            result = match teams::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("search", sub_matches)) => {
            result = match search::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }
        Some(("upgrade", sub_matches)) => {
            result = match upgrade::prepare(sub_matches) {
                Ok(cmd) => cmd.exec(),
                Err(err) => Err(err),
            };
        }

        _ => result = Err(Error::new(ErrorKind::InvalidInput, "No command provided")),
    }

    match result {
        Err(err) => {
            OutExtra::sum_failure(&err.to_string());
            exit(1);
        }
        Ok(_) => {
            OutExtra::sum_success("It was fun, wasn't it?");
        }
    }
}
