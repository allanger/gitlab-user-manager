use std::io::Error;

use clap::{arg, App, ArgMatches};

use crate::{cmd::Cmd, files};

pub(crate) fn add_remove_cmd() -> App<'static> {
    return App::new("remove")
        .alias("r")
        .about("Remove the team from the config file")
        .arg(arg!(<TEAM_NAME> "Name the team you're removing"));
}

struct RemoveCmd {
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

    Ok(RemoveCmd {
        team_name: team_name.unwrap().to_string(),
    })
}

impl<'a> Cmd<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };

        println!("Removing {} team", self.team_name);

        //  TODO: It shouldn't look that bad, I hope
        config.teams.retain(|t| t.name != self.team_name);

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
