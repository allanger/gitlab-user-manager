use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::{
    ArgAccess, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgGroupId, ArgNamespaceId, Args,
};
use crate::cmd::CmdOld;
use crate::gitlab::{GitlabActions, GitlabClient};
use crate::output::{out_message::OutMessage, out_spinner::OutSpinner};
use crate::types::v1::access_level::AccessLevel;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::namespace::Namespace;

pub(crate) struct AddGroupCmd {
    file_name: String,
    gitlab_group_id: u64,
    gitlab_namespace_id: u64,
    gitlab_client: Gitlab,
    access_level: AccessLevel,
}
pub(crate) fn add_add_namespace_cmd() -> Command<'static> {
    return Command::new("add-namespace")
        .alias("an")
        .about("Add a group access to namespace")
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgNamespaceId::add())
        .arg(ArgGroupId::add())
        .arg(ArgAccess::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_namespace_id = ArgNamespaceId::parse(sub_matches)?;

    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    // Connect to gitlab
    let gitlab_client: Gitlab =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gitlab_group_id = ArgNamespaceId::parse(sub_matches)?;

    let file_name = ArgFileName::parse(sub_matches)?;

    let access_level = ArgAccess::parse(sub_matches)?;

    Ok(AddGroupCmd {
        gitlab_namespace_id,
        gitlab_client,
        gitlab_group_id,
        file_name,
        access_level,
    })
}

impl<'a> CmdOld<'a> for AddGroupCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());

        OutMessage::message_info_with_alias("I'm getting data about the group from Gitlab");

        let namespace = gitlab.get_group_data_by_id(self.gitlab_namespace_id)?;

        for group in config_file.config.groups.iter_mut() {
            if group.id == self.gitlab_group_id {
                let spinner = OutSpinner::spinner_start(format!(
                    "Adding {} to {} as owner",
                    group.name, namespace.name
                ));
                let o = Namespace {
                    id: namespace.id,
                    name: namespace.name.to_string(),
                    url: namespace.web_url.to_string(),
                    access_level: self.access_level,
                };
                if group.namespaces.iter().any(|i| i.id == o.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "the group {} is already owner of this namespace: '{}'",
                            group.name, o.name
                        ),
                    ));
                }
                group.namespaces.extend([o]);
                spinner.spinner_success("Added".to_string());
            }
        }

        config_file.write(self.file_name.clone())
    }
}
