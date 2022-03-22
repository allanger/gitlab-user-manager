use crate::{
    args::{file_name::ArgFileName, large_out::ArgLargeOut, Args},
    cmd::CmdOld,
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

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
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

impl<'a> CmdOld<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        let total = &config_file.config.groups.len();
        for group in config_file.config.groups {
            let mut message = format!("{} - {}", group.id, group.name);
            if self.large_out {
                message.push_str(
                    format!(
                        "\nprojects: {:?}\ngroups: {:?}\n",
                        group.projects, group.namespaces
                    )
                    .as_str(),
                );
            }
            OutMessage::message_empty(message.as_str());
        }
        OutMessage::message_info_with_alias(format!("You've got {} groups here", total).as_str());
        Ok(())
    }
}
