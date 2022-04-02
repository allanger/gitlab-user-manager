use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::args::file_name::ArgFileName;
use crate::args::namespace_id::ArgNamespaceId;
use crate::args::user_id::ArgUserId;
use crate::args::Args;
use crate::cmd::CmdOld;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;

pub(crate) struct RemoveGroupCmd {
    gitlab_user_id: u64,
    gitlab_group_id: u64,
    file_name: String,
}
pub(crate) fn add_remove_namespace_cmd() -> Command<'static> {
    return Command::new("remove-namespace")
        .alias("rn")
        .about("Remove user from namespace")
        .arg(ArgUserId::add())
        .arg(ArgNamespaceId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_group_id = ArgNamespaceId::parse(sub_matches)?;

    let gitlab_user_id = ArgUserId::parse(sub_matches)?;
    let file_name = ArgFileName::parse(sub_matches)?;

    Ok(RemoveGroupCmd {
        gitlab_group_id,
        gitlab_user_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveGroupCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = ConfigFile::read(self.file_name.clone())?;

        for u in config_file.config.users.iter_mut() {
            if u.id == self.gitlab_user_id {
                for (i, o) in u.namespaces.iter().enumerate() {
                    if o.id == self.gitlab_group_id {
                        OutMessage::message_info_clean(
                            format!("Removing ownership on {} for user {}", o.name, u.name)
                                .as_str(),
                        );

                        u.namespaces.remove(i);
                        break;
                    }
                }
            }
        }
        config_file.write(self.file_name.clone())
    }
}
