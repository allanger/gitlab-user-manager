use std::io::Error;

use clap::{arg, ArgMatches, Command};

use crate::{cmd::Cmd, files, output::OutMessage};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
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
            Err(err) => return Err(err),
        };
        config.teams.retain(|t| t.name != self.team_name);

        let _ = match files::write_config(config) {
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
