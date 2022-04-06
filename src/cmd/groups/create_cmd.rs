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
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    let gitlab_client: Gitlab =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gitlab_group_id = ArgGroupId::parse(sub_matches)?;

    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(CreateCmd {
        gitlab_group_id,
        gitlab_client,
        file_name,
    })
}

impl<'a> CmdOld<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");

        let group = gitlab.get_group_data_by_id(self.gitlab_group_id)?;

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
        config_file.write(self.file_name.clone())
    }
}
