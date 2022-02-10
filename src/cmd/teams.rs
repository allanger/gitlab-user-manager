use std::io::Error;

use clap::{App, ArgMatches};

use self::create::add_create_cmd;

use super::Cmd;

pub(crate) fn add_teams_cmd() -> App<'static> {
    // Register command
    return App::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .subcommand(add_create_cmd());
    // .subcommand(list_teams())
    // .subcommand(remove_team())
    // .subcommand(add_project_to_team())
    // .subcommand(remove_project_from_team());
}

pub(crate) struct TeamsCmd<'a> {
    teams_sub: Option<(&'a str, &'a ArgMatches)>,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    Ok(TeamsCmd {
        teams_sub: sub_matches.subcommand(),
    })
}

impl<'a> Cmd<'a> for TeamsCmd<'a> {
    fn exec(&self) -> Result<(), Error> {
        match self.teams_sub.search_sub {
            Some(("create", sub_matches)) => {
                return create(
                    sub_matches
                        .value_of("TEAM_NAME")
                        .expect("Team name is required"),
                )
            }
            Some(("remove", sub_matches)) => {
                return remove(
                    sub_matches
                        .value_of("TEAM_NAME")
                        .expect("Team name is required"),
                )
            }
            Some(("list", _)) => {
                return list();
            }
            Some(("add-project", sub_matches)) => add_project(sub_matches),
            Some(("remove-project", sub_matches)) => remove_project(sub_matches),
            _ => return Ok(()),
            None => todo!(),
        }
    }
}

mod create {
    use clap::{arg, App};

    fn add_create_cmd() -> App<'static> {
        return App::new("create")
            .alias("c")
            .about("Add a team to the config file")
            .arg(arg!(<TEAM_NAME> "Name the team you're creating"));
    }

    struct CreateCmd;
    pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
        Ok(CreateCmd)
    }

    impl<'a> Cmd<'a> for TeamsCmd<'a> {
        fn exec(&self) -> Result<(), Error> {
            match self.teams_sub.search_sub {
                Some(("create", sub_matches)) => {
                    return create(
                        sub_matches
                            .value_of("TEAM_NAME")
                            .expect("Team name is required"),
                    )
                }
                Some(("remove", sub_matches)) => {
                    return remove(
                        sub_matches
                            .value_of("TEAM_NAME")
                            .expect("Team name is required"),
                    )
                }
                Some(("list", _)) => {
                    return list();
                }
                Some(("add-project", sub_matches)) => add_project(sub_matches),
                Some(("remove-project", sub_matches)) => remove_project(sub_matches),
                _ => return Ok(()),
                None => todo!(),
            }
        }
    }
}
mod remove {}
mod list {}
mod add_project {}
mod remove_project {}
