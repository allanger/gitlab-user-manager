use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, team_name::ArgTeamName, Args},
    cmd::CmdOld,
    output::out_message::OutMessage,
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

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let team_name = ArgTeamName::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(RemoveCmd {
        team_name,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        config_file
            .config
            .teams
            .retain(|t| t.name != self.team_name);

        match config_file.write(self.file_name.clone()) {
            Ok(()) => {
                OutMessage::message_info_clean(
                    format!("The team is removed: {}", self.team_name).as_str(),
                );
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
