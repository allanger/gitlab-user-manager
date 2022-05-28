use crate::{
    args::{ArgFileName, ArgLargeOut, Args},
    cmd::{Cmd, CmdOld},
    service::v1,
};
use clap::{ArgMatches, Command};
use std::io::Error;
use std::io::Result;

pub(crate) fn add_list_cmd() -> Command<'static> {
    return Command::new("list")
        .alias("l")
        .about("List teams from config file")
        .arg(ArgFileName::add())
        .arg(ArgLargeOut::add());
}
pub(crate) struct ListCmd {
    file_name: String,
    large_out: bool,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl Cmd> {
    let file_name = ArgFileName::parse(sub_matches)?;
    let large_out: bool = ArgLargeOut::parse(sub_matches)?;

    Ok(ListCmd {
        file_name,
        large_out,
    })
}

impl Cmd for ListCmd {
    type CmdType = ListCmd;

    fn add() -> Command<'static> {
        Command::new("create")
            .alias("l")
            .about("Add user to the config file")
            .arg(ArgFileName::add())
            .arg(ArgLargeOut::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> std::io::Result<Self::CmdType> {
        let file_name = ArgFileName::parse(sub_matches)?;
        let large_out: bool = ArgLargeOut::parse(sub_matches)?;

        Ok(ListCmd {
            file_name,
            large_out,
        })
    }

    fn exec(&self) -> std::io::Result<()> {
        todo!()
    }
}

impl ListCmd {
    fn exec_v1(&self) -> Result<()> {
        let mut svc = v1::users::UsersService::new(
            self.file_name.clone(),
            self.file_name.clone(),
            self.gitlab_user_id,
        );
        svc.list()?.write_state()
    }
}
