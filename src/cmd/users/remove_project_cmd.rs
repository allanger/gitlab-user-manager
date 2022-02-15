use std::io::{Error, ErrorKind};

use clap::{arg, App, ArgMatches};

use crate::{
    cmd::{arg_gitlab_token, arg_gitlab_url, arg_project_id, Cmd},
    files,
};

pub(crate) struct RemoveProjectCmd {
    gitlab_user_id: u64,
    gitlab_project_id: u64,
}
pub(crate) fn add_remove_project_cmd() -> App<'static> {
    return App::new("remove-project")
        .alias("rp")
        .about("Remove user from the project")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_project_id());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_project_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };
    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(pid) => pid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    Ok(RemoveProjectCmd {
        gitlab_project_id,
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for RemoveProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };
        for u in config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, p) in u.projects.iter().enumerate() {
                    if p.id == self.gitlab_project_id {
                        println!("removing user {} from project {}", u.name, p.name);
                        u.projects.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
