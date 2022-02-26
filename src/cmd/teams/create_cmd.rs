use std::io::{Error, ErrorKind};

use clap::{arg, Command, ArgMatches};

use crate::{cmd::Cmd, files, types};

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add a team to the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're creating"));
}

struct CreateCmd {
    team_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let team_name = sub_matches.value_of("TEAM_NAME").ok_or(Error::new(
        std::io::ErrorKind::PermissionDenied,
        "team name is not specified",
    ));
    if team_name.is_err() {
        return Err(team_name.err().unwrap());
    }

    Ok(CreateCmd {
        team_name: team_name.unwrap().to_string(),
    })
}

impl<'a> Cmd<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };

        let new_team = types::team::Team {
            name: self.team_name.to_string(),
            ..Default::default()
        };
        //  TODO: It shouldn't look that bad, I hope
        if config.teams.iter().any(|i| i.name == new_team.name) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "team with this name already exists",
            ));
        }

        config.teams.extend([new_team]);

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
