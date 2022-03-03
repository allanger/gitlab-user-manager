use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::args::group_id::ArgGroupId;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::files;
use crate::output::OutMessage;

pub(crate) struct RemoveOwnershipCmd {
    gitlab_user_id: u64,
    gitlab_group_id: u64,
}
pub(crate) fn add_remove_ownership_cmd() -> Command<'static> {
    return Command::new("remove-ownership")
        .alias("ro")
        .about("Remove an ownership from the user")
        .arg(ArgUserId::add())
        .arg(ArgGroupId::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_group_id = match ArgGroupId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };

    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
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
                        OutMessage::message_info_clean(
                            format!("Removing ownership on {} for user {}", o.name, u.name)
                                .as_str(),
                        );

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
