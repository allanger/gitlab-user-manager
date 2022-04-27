use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::{
    ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgNamespaceId, ArgUserId, Args,
};
use crate::cmd::CmdOld;
use crate::gitlab::{GitlabActions, GitlabClient};
use crate::output::{out_message::OutMessage, out_spinner::OutSpinner};
use crate::types::v1::AccessLevel;
use crate::types::v1::ConfigFile;
use crate::types::v1::Namespace;

pub(crate) struct AddGroupCmd {
    file_name: String,
    gitlab_user_id: u64,
    gitlab_group_id: u64,
    gitlab_client: Gitlab,
    access_level: AccessLevel,
}
pub(crate) fn add_add_namespace_cmd() -> Command<'static> {
    return Command::new("add-namespace")
        .alias("an")
        .about("Add a user access to namespace")
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgNamespaceId::add())
        .arg(ArgUserId::add())
        .arg(ArgAccess::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    // Connect to gitlab
    let gitlab_client: Gitlab =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gitlab_group_id = ArgNamespaceId::parse(sub_matches)?;

    let gitlab_user_id: u64 =
        ArgUserId::parse(sub_matches).map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

    let access_level = ArgAccess::parse(sub_matches)?;

    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(AddGroupCmd {
        gitlab_group_id,
        gitlab_client,
        gitlab_user_id,
        file_name,
        access_level,
    })
}

impl<'a> CmdOld<'a> for AddGroupCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;
        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());

        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");

        let namespace = gitlab.get_group_data_by_id(self.gitlab_group_id)?;

        for user in config_file.config_mut().users.iter_mut() {
            if user.id == self.gitlab_user_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as owner",
                    user.name, namespace.name
                ));
                let o = Namespace {
                    id: namespace.id,
                    name: namespace.name.to_string(),
                    url: namespace.web_url.to_string(),
                    access_level: self.access_level,
                };
                if user.namespaces.iter().any(|i| i.id == o.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the user {} is already owner of this group: '{}'",
                            user.name, o.name
                        ),
                    ));
                }
                user.namespaces.extend([o]);
                spinner.spinner_success("Added".to_string());
            }
        }
        config_file.write(self.file_name.clone())
    }
}
