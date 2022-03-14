use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::file_name::ArgFileName;
use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::group_id::ArgGroupId;
use crate::args::Args;
use crate::cmd::CmdOld;
use crate::gitlab::GitlabActions;
use crate::gitlab::GitlabClient;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::group::Group;

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add group to the config file")
        .arg(ArgGroupId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgFileName::add());
}

struct CreateCmd {
    gitlab_group_id: u64,
    gitlab_client: Gitlab,
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_token = match ArgGitlabToken::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let gitlab_url = match ArgGitlabUrl::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let gitlab_client: Gitlab = match Gitlab::new(gitlab_url, gitlab_token)
    {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_group_id = match ArgGroupId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(CreateCmd {
        gitlab_group_id,
        gitlab_client,
        file_name,
    })
}

impl<'a> CmdOld<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");
        
        let group = match gitlab.get_group_data_by_id(self.gitlab_group_id) {
            Ok(u) => u,
            Err(err) => return Err(err),
        };

        let new_user = Group {
            id: self.gitlab_group_id,
            name: group.name.to_string(),
            ..Default::default()
        };

        if config_file
            .config
            .groups
            .iter()
            .any(|i| i.id == self.gitlab_group_id)
        {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("Group {} is already in the config file", new_user.name),
            ));
        } else {
            config_file.config.groups.extend([new_user]);
            OutMessage::message_info_clean(
                format!("Group {} is added to the config", group.name).as_str(),
            );
        }

        let _ = match config_file.write(self.file_name.clone()) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
