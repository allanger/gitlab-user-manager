use std::io::{Error, ErrorKind};

use clap::{ArgMatches, Command};

use crate::args::file_name::ArgFileName;
use crate::args::group_id::ArgGroupId;
use crate::args::namespace_id::ArgNamespaceId;
use crate::args::Args;
use crate::cmd::CmdOld;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;

pub(crate) struct RemoveGroupCmd {
    gitlab_group_id: u64,
    gitlab_namespace_id: u64,
    file_name: String,
}
pub(crate) fn add_remove_namespace_cmd() -> Command<'static> {
    return Command::new("remove-namespace")
        .alias("rn")
        .about("Remove group from namespace")
        .arg(ArgGroupId::add())
        .arg(ArgNamespaceId::add())
        .arg(ArgFileName::add());
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>, Error> {
    let gitlab_namespace_id = match ArgNamespaceId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string())),
    };

    let gitlab_group_id = match ArgGroupId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(RemoveGroupCmd {
        gitlab_namespace_id,
        gitlab_group_id,
        file_name,
    })
}

impl<'a> CmdOld<'a> for RemoveGroupCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        for g in config_file.config.groups.iter_mut() {
            if g.id == self.gitlab_group_id {
                for (i, o) in g.namespaces.iter().enumerate() {
                    if o.id == self.gitlab_namespace_id {
                        OutMessage::message_info_clean(
                            format!("Removing ownership on {} for user {}", o.name, g.name)
                                .as_str(),
                        );

                        g.namespaces.remove(i);
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
