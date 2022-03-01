use crate::{cmd::Cmd, files, output::OutMessage};
use clap::Command;

use std::io::Error;

pub(crate) fn add_list_cmd() -> Command<'static> {
    return Command::new("list")
        .alias("l")
        .about("List teams from config file");
}
struct ListCmd;

pub(crate) fn prepare<'a>() -> Result<impl Cmd<'a>, Error> {
    Ok(ListCmd)
}

impl<'a> Cmd<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        for team in config.teams.iter() {
            OutMessage::message_empty(format!("{}: {:?}\n", team.name, team.projects).as_str());
        }
        Ok(())
    }
}
