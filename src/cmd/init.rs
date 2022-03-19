use std::io::Result;

use clap::{ArgMatches, Command};

use crate::{
    args::{
        file_name::ArgFileName, gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl,
        group_list::ArgGroupList, Args,
    },
    gitlab::GitlabApi,
    service::init::InitService,
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
            .after_help("$ gum init -g 111 222 -f gum-config-example.yaml ## where 111 and 222 are groups ids")
            .before_help("Use this command if you want to be sure that you're starting to use gum the right way")
            .arg(ArgFileName::add())
            .arg(ArgGroupList::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> Result<InitCmd> {
        Ok(InitCmd {
            file_name: ArgFileName::parse(sub_matches)?.value(),
            group_list: ArgGroupList::parse(sub_matches)?.value().to_vec(),
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?.value(),
            gitlab_token: ArgGitlabToken::parse(sub_matches)?.value(),
        })
    }

    fn exec(&self) -> Result<()> {
        InitService::new(GitlabApi::new(&self.gitlab_url, &self.gitlab_token)?)
            .parse_groups(&self.group_list)?
            .save(&self.file_name)
    }
}
