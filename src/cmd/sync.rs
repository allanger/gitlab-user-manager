use crate::{
    cmd::Cmd,
    gitlab::GitlabApi,
    output::out_message::OutMessage,
    service::v1,
    types::{
        common::{Version, Versions},
        v1::ConfigFile,
    },
};
use std::io::Result;

use clap::{ArgMatches, Command};

use crate::args::{
    ArgDryRun, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgStateDestination, ArgStateSource,
    ArgWriteState, Args,
};

pub(crate) struct SyncCmd {
    dry_run: bool,
    gitlab_token: String,
    gitlab_url: String,
    file_name: String,
    write_state: bool,
    state_destination: String,
    state_source: String,
}

impl<'a> Cmd for SyncCmd {
    type CmdType = SyncCmd;
    fn add() -> Command {
        Command::new("sync")
            .about("Sync your config file with GitLab and generate the state file")
            .alias("s")
            .after_help("$ gum sync -f gum-config-example.yaml --dry-run")
            .before_help("Use this command if you want to apply changes in your configuration file to GitLab")
            .arg(ArgDryRun::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
            .arg(ArgStateDestination::add())
            .arg(ArgStateSource::add())
            .arg(ArgWriteState::add())
    }

    fn prepare(sub_matches: &ArgMatches) -> Result<Self> {
        Ok(SyncCmd {
            dry_run: ArgDryRun::parse(sub_matches)?,
            file_name: ArgFileName::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            state_destination: ArgStateDestination::parse(sub_matches)?,
            state_source: ArgStateSource::parse(sub_matches)?,
            write_state: ArgWriteState::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> Result<()> {
        match ConfigFile::read(self.file_name.clone())?.get_version()? {
            Versions::V1 => self.exec_v1(),
        }
    }
}

impl SyncCmd {
    fn exec_v1(&self) -> Result<()> {
        OutMessage::message_info_with_alias("You may be using the an outdated config version");
        let mut svc = v1::SyncService::new(
            self.file_name.clone(),
            GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?,
            self.state_source.clone(),
            self.state_destination.clone(),
            self.write_state,
        );
        svc.read_config()?
            .create_states()?
            .compare()?
            .apply(self.dry_run)?
            .update_state()?
            .write_state(self.dry_run)
    }
}
