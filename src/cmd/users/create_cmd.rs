use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::{ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgUserId, Args};
use crate::cmd::CmdOld;
use crate::gitlab::GitlabActions;
use crate::gitlab::GitlabClient;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::user::User;

pub(crate) fn add_create_cmd() -> Command<'static> {
    return Command::new("create")
        .alias("c")
        .about("Add user to the config file")
        .arg(ArgUserId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgFileName::add());
}

struct CreateCmd {
    gitlab_user_id: u64,
    gitlab_client: Gitlab,
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    let gitlab_client: Gitlab =
        Gitlab::new(gitlab_url, gitlab_token).map_err(|err| Error::new(ErrorKind::Other, err))?;

    let gitlab_user_id = ArgUserId::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(CreateCmd {
        gitlab_user_id,
        gitlab_client,
        file_name,
    })
}

impl<'a> CmdOld<'a> for CreateCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        OutMessage::message_info_with_alias("I'm getting data about the user from Gitlab");
        let user = gitlab.get_user_data_by_id(self.gitlab_user_id)?;

        let new_user = User {
            id: self.gitlab_user_id,
            name: user.name.to_string(),
            ..Default::default()
        };

        if config_file
            .config
            .users
            .iter()
            .any(|i| i.id == self.gitlab_user_id)
        {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("User {} is already in the config file", new_user.name),
            ));
        } else {
            config_file.config.users.extend([new_user]);
            OutMessage::message_info_clean(
                format!("User {} is added to the config", user.name).as_str(),
            );
        }

        config_file.write(self.file_name.clone())
    }
}
