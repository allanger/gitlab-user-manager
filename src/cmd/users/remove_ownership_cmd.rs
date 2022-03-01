use std::io::{Error, ErrorKind};

use clap::{arg, Command, ArgMatches};

use crate::cmd::Cmd;
use crate::{
    cmd::args::{arg_gitlab_token, arg_gitlab_url, arg_group_id},
    files,
};

pub(crate) struct RemoveOwnershipCmd {
    gitlab_user_id: u64,
    gitlab_group_id: u64,
}
pub(crate) fn add_remove_ownership_cmd() -> Command<'static> {
    return Command::new("remove-ownership")
        .alias("ro")
        .about("Remove an ownership from the user")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"))
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url())
        .arg(arg_group_id());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_group_id: u64 = match sub_matches.value_of_t("project-id") {
        Ok(pid) => pid,
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };
    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(pid) => pid,
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };

    Ok(RemoveOwnershipCmd {
        gitlab_group_id,
        gitlab_user_id,
    })
}

impl<'a> Cmd<'a> for RemoveOwnershipCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        for u in config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, o) in u.ownerships.iter().enumerate() {
                    if o.id == self.gitlab_group_id {
                        println!("removing ownership on {} for user {}", o.name, u.name);
                        u.ownerships.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
