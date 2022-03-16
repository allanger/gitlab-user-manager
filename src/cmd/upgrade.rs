use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use self_update::{backends::github::Update, cargo_crate_version};

use crate::{
    args::{no_confirm::ArgNoConfirm, Args},
    cmd::CmdOld,
    output::out_message::OutMessage,
};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_upgrade_cmd() -> Command<'static> {
    return Command::new("upgrade")
        .about("Update current gum with a newer version")
        .arg(ArgNoConfirm::add());
}

pub(crate) struct UpgradeCmd {
    no_confirm: bool,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let no_confirm: bool = ArgNoConfirm::parse(sub_matches).unwrap().value();
    Ok(UpgradeCmd { no_confirm })
}

impl<'a> CmdOld<'a> for UpgradeCmd {
    fn exec(&self) -> Result<(), Error> {
        let status = match Update::configure()
            .repo_owner("allanger")
            .repo_name("gitlab-user-manager")
            .bin_name("gum")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .no_confirm(self.no_confirm)
            .build()
            .unwrap()
            .update()
        {
            Ok(s) => s,
            Err(err) => return Err(Error::new(ErrorKind::Other, err.to_string())),
        };
        OutMessage::message_empty(format!("Update status: `{}`!", status.version()).as_str());
        Ok(())
    }
}
