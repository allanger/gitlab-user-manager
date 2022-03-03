use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, user_id::ArgUserId, Args},
    cmd::Cmd,
    output::OutMessage,
    types::v1::{config_file::ConfigFile, user::User},
};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove user from the config file")
        .arg(ArgUserId::add())
        .arg(ArgFileName::add());
}

struct RemoveCmd {
    gitlab_user_id: u64,
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_user_id = match ArgUserId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveCmd {
        gitlab_user_id,
        file_name,
    })
}

impl<'a> Cmd<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for (i, user) in config_file.config.users.iter().enumerate() {
            if user.id == self.gitlab_user_id {
                let u = User {
                    id: user.id,
                    name: user.name.to_string(),
                    ..Default::default()
                };
                OutMessage::message_info_clean(
                    format!("removing user {} from config", u.name).as_str(),
                );
                config_file.config.users.remove(i);
                break;
            }
        }

        let _ = match config_file.write(self.file_name.clone()) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
