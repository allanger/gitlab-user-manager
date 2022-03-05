use crate::{
    args::{file_name::ArgFileName, large_out::ArgLargeOut, Args},
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
        .arg(ArgFileName::add())
        .arg(ArgLargeOut::add());
}
struct ListCmd {
    file_name: String,
    large_out: bool,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let large_out: bool = ArgLargeOut::parse(sub_matches).unwrap().value();

    Ok(ListCmd {
        file_name,
        large_out,
    })
}

impl<'a> Cmd<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for user in config_file.config.users {
            let mut message = format!("{} - {}", user.id, user.name);
            if self.large_out {
                message.push_str(
                    format!(
                        "\nprojects: {:?}\nteams: {:?}\nownerships: {:?}\n",
                        user.projects, user.teams, user.ownerships
                    )
                    .as_str(),
                );
            }
            OutMessage::message_empty(message.as_str());
        }
        Ok(())
    }
}
