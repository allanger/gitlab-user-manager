use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::{
    args::{team_name::ArgTeamName, Args},
    cmd::Cmd,
    files,
    output::OutMessage,
    types,
};

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add a team to the config file")
        .arg(ArgTeamName::add());
}

struct CreateCmd {
    team_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(CreateCmd { team_name })
}

impl<'a> Cmd<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        let new_team = types::team::Team {
            name: self.team_name.to_string(),
            ..Default::default()
        };
        if config.teams.iter().any(|i| i.name == new_team.name) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "team with this name already exists",
            ));
        }

        config.teams.extend([new_team]);

        let _ = match files::write_config(config) {
            Ok(()) => {
                OutMessage::message_info_clean(
                    format!("New team is created: {}", self.team_name).as_str(),
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        };
    }
}
