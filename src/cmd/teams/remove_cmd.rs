use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{team_name::ArgTeamName, Args},
    cmd::Cmd,
    files,
    output::OutMessage,
};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove the team from the config file")
        .arg(ArgTeamName::add());
}

struct RemoveCmd {
    team_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveCmd { team_name })
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
