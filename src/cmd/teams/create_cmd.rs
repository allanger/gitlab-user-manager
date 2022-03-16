use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, team_name::ArgTeamName, Args},
    cmd::CmdOld,
    output::out_message::OutMessage,
    types::v1::{config_file::ConfigFile, team::Team},
};

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add a team to the config file")
        .arg(ArgFileName::add())
        .arg(ArgTeamName::add());
}

struct CreateCmd {
    file_name: String,
    team_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(CreateCmd {
        team_name,
        file_name,
    })
}

impl<'a> CmdOld<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        let new_team = Team {
            name: self.team_name.to_string(),
            ..Default::default()
        };
        if config_file
            .config
            .teams
            .iter()
            .any(|i| i.name == new_team.name)
        {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "team with this name already exists",
            ));
        }

        config_file.config.teams.extend([new_team]);

        match config_file.write(self.file_name.clone()) {
            Ok(()) => {
                OutMessage::message_info_clean(
                    format!("New team is created: {}", self.team_name).as_str(),
                );
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
