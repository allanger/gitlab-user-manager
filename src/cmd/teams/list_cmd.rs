use crate::{
    args::{ArgFileName, Args},
    cmd::CmdOld,
    output::{out_extra::OutExtra, out_message::OutMessage},
    types::v1::ConfigFile,
};
use clap::{ArgMatches, Command};
use console::style;

use std::io::Error;

pub(crate) fn add_list_cmd() -> Command<'static> {
    return Command::new("list")
        .alias("l")
        .about("List teams from config file")
        .arg(ArgFileName::add());
}
struct ListCmd {
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let file_name = ArgFileName::parse(sub_matches)?;
    Ok(ListCmd { file_name })
}

impl<'a> CmdOld<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        let total = &config_file.config().teams.len();

        for team in config_file.config().teams.iter() {
            OutMessage::message_empty(format!("{}: {:?}\n", team.name, team.projects).as_str());
        }
        OutExtra::empty_line();
        OutMessage::message_info_with_alias(
            format!("You've got {} teams here", style(total).bold().underlined()).as_str(),
        );

        Ok(())
    }
}
