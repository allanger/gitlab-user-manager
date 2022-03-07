use crate::{
    args::{file_name::ArgFileName, Args},
    cmd::Cmd,
    output::out_message::OutMessage,
    types::v1::config_file::ConfigFile,
};
use clap::{ArgMatches, Command};

use std::io::Error;

pub(crate) fn add_list_cmd() -> Command<'static> {
    return Command::new("list")
        .alias("l")
        .about("List teams from config file")
        .arg(ArgFileName::add());
}
struct ListCmd {
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(ListCmd { file_name })
}

impl<'a> Cmd<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for team in config_file.config.teams.iter() {
            OutMessage::message_empty(format!("{}: {:?}\n", team.name, team.projects).as_str());
        }
        Ok(())
    }
}
