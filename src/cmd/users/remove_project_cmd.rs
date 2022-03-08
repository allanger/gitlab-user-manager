use std::io::Error;

use clap::{ArgMatches, Command};

use crate::args::file_name::ArgFileName;
use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::project_id::ArgProjectId;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;

pub(crate) struct RemoveProjectCmd {
    gitlab_user_id: u64,
    gitlab_project_id: u64,
    file_name: String,
}
pub(crate) fn add_remove_project_cmd() -> Command<'static> {
    return Command::new("remove-project")
        .alias("rp")
        .about("Remove user from the project")
        .arg(ArgUserId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgProjectId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_project_id: u64 = match ArgProjectId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveProjectCmd {
        gitlab_project_id,
        gitlab_user_id,
        file_name,
    })
}

impl<'a> Cmd<'a> for RemoveProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for u in config_file.config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, p) in u.projects.iter().enumerate() {
                    if p.id == self.gitlab_project_id {
                        OutMessage::message_info_clean(
                            format!("removing user {} from project {}", u.name, p.name).as_str(),
                        );

                        u.projects.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match config_file.write(self.file_name.clone()) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
