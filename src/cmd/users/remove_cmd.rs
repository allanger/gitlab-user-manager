use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{user_id::ArgUserId, Args},
    cmd::Cmd,
    files,
    output::OutMessage,
    types::user::User,
};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove user from the config file")
        .arg(ArgUserId::add());
}

struct RemoveCmd {
    gitlab_user_id: u64,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveCmd { gitlab_user_id })
}

impl<'a> Cmd<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for (i, user) in config.users.iter().enumerate() {
            if user.id == self.gitlab_user_id {
                let u = User {
                    id: user.id,
                    name: user.name.to_string(),
                    ..Default::default()
                };
                OutMessage::message_info_clean(
                    format!("removing user {} from config", u.name).as_str(),
                );
                config.users.remove(i);
                break;
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
