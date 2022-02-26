use std::io::{Error, ErrorKind};

use clap::{arg, Command, ArgMatches};

use crate::{cmd::Cmd, files, types::user::User};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove user from the config file")
        .arg(arg!(<GITLAB_USER_ID> "Provide the GitLab user ID"));
}

struct RemoveCmd {
    gitlab_user_id: u64,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_user_id: u64 = match sub_matches.value_of_t("GITLAB_USER_ID") {
        Ok(uid) => uid,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };

    Ok(RemoveCmd { gitlab_user_id })
}

impl<'a> Cmd<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };

        for (i, user) in config.users.iter().enumerate() {
            if user.id == self.gitlab_user_id {
                let u = User {
                    id: user.id,
                    name: user.name.to_string(),
                    ..Default::default()
                };
                println!("removing user {} from config", u.name);
                config.users.remove(i);
                break;
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
