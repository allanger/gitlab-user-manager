use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, team_name::ArgTeamName, Args},
    cmd::Cmd,
    output::OutMessage,
    types::v1::config_file::ConfigFile,
};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove the team from the config file")
        .arg(ArgTeamName::add())
        .arg(ArgFileName::add());
}

struct RemoveCmd {
    team_name: String,
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveCmd {
        team_name,
        file_name,
    })
}

impl<'a> Cmd<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        config_file
            .config
            .teams
            .retain(|t| t.name != self.team_name);

        match config_file.write(self.file_name.clone()) {
            Ok(()) => {
                OutMessage::message_info_clean(
                    format!("The team is removed: {}", self.team_name).as_str(),
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        };
    }
}
