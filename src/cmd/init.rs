use std::io::Result;

use clap::{ArgMatches, Command};

use crate::{
    args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgGroupList, Args},
    gitlab::GitlabApi,
    service::v1,
};

use super::Cmd;

pub(crate) struct InitCmd {
    file_name: String,
    group_list: Vec<u64>,
    gitlab_url: String,
    gitlab_token: String,
}

impl Cmd for InitCmd {
    type CmdType = InitCmd;

    fn add() -> Command<'static> {
        Command::new("init")
            .about("Generate a config file")
            .alias("i")
            .after_help("$ gum init -g 111 222 -f gum-config-example.yaml -- where 111 and 222 are groups ids")
            .before_help("Use this command if you want to be sure that you're starting to use gum the right way")
            .arg(ArgFileName::add())
            .arg(ArgGroupList::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> Result<InitCmd> {
        Ok(InitCmd {
            file_name: ArgFileName::parse(sub_matches)?,
            group_list: ArgGroupList::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
        })
    }

    fn exec(&self) -> Result<()> {
        self.exec_v1()
    }
}

impl InitCmd {
    fn exec_v1(&self) -> Result<()> {
        v1::InitService::new(GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?)
            .generate_config(&self.group_list)?
            .save(&self.file_name)
    }
}
