use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use self_update::cargo_crate_version;

use crate::cmd::Cmd;

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_self_update_cmd() -> Command<'static> {
    return Command::new("self-update").about("Update current gum with a newer version");
}

pub(crate) struct SelfUpdatedCmd;

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    Ok(SelfUpdatedCmd)
}

impl<'a> Cmd<'a> for SelfUpdatedCmd {
    fn exec(&self) -> Result<(), Error> {
      heim::
        let status = self_update::backends::github::Update::configure()
            .repo_owner("allanger")
            .repo_name("gitlab-user-manager")
            .bin_name("gum")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .target(target)
            .build()
            .unwrap()
            .update()
            .unwrap();
        println!("Update status: `{}`!", status.version());
        Ok(())
    }
}
