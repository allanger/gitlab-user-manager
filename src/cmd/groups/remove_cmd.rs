use std::io::Error;

use clap::{ArgMatches, Command};

use crate::{
    args::{file_name::ArgFileName, group_id::ArgGroupId, Args},
    cmd::CmdOld,
    output::out_message::OutMessage,
    types::v1::{config_file::ConfigFile, user::User},
};

pub(crate) fn add_remove_cmd() -> Command<'static> {
    return Command::new("remove")
        .alias("r")
        .about("Remove group from the config file")
        .arg(ArgGroupId::add())
        .arg(ArgFileName::add());
}

struct RemoveCmd {
    gitlab_group_id: u64,
    file_name: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_group_id = match ArgGroupId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveCmd {
        gitlab_group_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for (i, user) in config_file.config.groups.iter().enumerate() {
            if user.id == self.gitlab_group_id {
                let u = User {
                    id: user.id,
                    name: user.name.to_string(),
                    ..Default::default()
                };
                OutMessage::message_info_clean(
                    format!("removing group {} from config", u.name).as_str(),
                );
                config_file.config.groups.remove(i);
                break;
            }
        }

        let _ = match config_file.write(self.file_name.clone()) {
            Ok(()) => return Ok(()),
            Err(err) => return Err(err),
        };
    }
}
