use crate::{cmd::Cmd, files};
use clap::App;

use std::io::Error;

pub(crate) fn add_list_cmd() -> App<'static> {
    return App::new("list")
        .alias("l")
        .about("List teams from config file");
}
struct ListCmd;

pub(crate) fn prepare<'a>() -> Result<impl Cmd<'a>, Error> {
    Ok(ListCmd)
}

impl<'a> Cmd<'a> for ListCmd {
    fn exec(&self) -> Result<(), Error> {
        let config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };

        for user in config.users {
            println!("{} - {}", user.id, user.name);
        }
        Ok(())
    }
}