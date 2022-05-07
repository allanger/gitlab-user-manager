use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{ArgFileName, ArgUserId, Args},
    cmd::CmdOld,
    output::out_message::OutMessage,
    types::v1::{ConfigFile, User},
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

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_user_id = ArgUserId::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(RemoveCmd {
        gitlab_user_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        for (i, user) in config_file.config().users.iter().enumerate() {
            if user.id == self.gitlab_user_id {
                let u = User {
                    id: user.id,
                    name: user.name.to_string(),
                    ..Default::default()
                };
                OutMessage::message_info_clean(
                    format!("removing user {} from config", u.name).as_str(),
                );
                config_file.config_mut().users.remove(i);
                break;
            }
        }
        config_file.write(self.file_name.clone())
    }
}
