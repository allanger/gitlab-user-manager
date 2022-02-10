use std::io::Error;

use clap::{App, ArgMatches};

use self::{
    add_project_cmd::add_add_project_cmd, create_cmd::add_create_cmd, list_cmd::add_list_cmd,
    remove_cmd::add_remove_cmd, remove_project_cmd::add_remove_project_cmd,
};

use super::Cmd;

pub(crate) fn add_teams_cmd() -> App<'static> {
    // Register command
    return App::new("teams")
        .aliases(&["t", "team"])
        .about("Manage GUM teams")
        .subcommand(add_create_cmd())
        .subcommand(add_list_cmd())
        .subcommand(add_remove_cmd())
        .subcommand(add_add_project_cmd())
        .subcommand(add_remove_project_cmd());
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
        let result;
        match self.teams_sub {
            Some(("create", sub_matches)) => {
                result = match create_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove", sub_matches)) => {
                result = match remove_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("list", _)) => {
                result = match list_cmd::prepare() {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("add-project", sub_matches)) => {
                result = match add_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }
            Some(("remove-project", sub_matches)) => {
                result = match remove_project_cmd::prepare(sub_matches) {
                    Ok(cmd) => cmd.exec(),
                    Err(_error) => Err(_error),
                }
            }

            _ => return Ok(()),
        }
        result
    }
}

mod create_cmd {
    use std::io::{Error, ErrorKind};

    use clap::{arg, App, ArgMatches};

    use crate::{cmd::Cmd, pkg::config, types::types};

    pub(crate) fn add_create_cmd() -> App<'static> {
        return App::new("create")
            .alias("c")
            .about("Add a team to the config file")
            .arg(arg!(<TEAM_NAME> "Name the team you're creating"));
    }

    struct CreateCmd {
        team_name: String,
    }

    pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
        let team_name = sub_matches.value_of("TEAM_NAME").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "team name is not specified",
        ));
        if team_name.is_err() {
            return Err(team_name.err().unwrap());
        }

        Ok(CreateCmd {
            team_name: team_name.unwrap().to_string(),
        })
    }

    impl<'a> Cmd<'a> for CreateCmd {
        fn exec(&self) -> Result<(), Error> {
            let mut config = match config::read_config() {
                Ok(c) => c,
                Err(_error) => return Err(_error),
            };

            let new_team = types::Team {
                name: self.team_name.to_string(),
                projects: None,
            };
            //  TODO: It shouldn't look that bad, I hope
            if config
                .teams
                .as_mut()
                .unwrap()
                .iter()
                .any(|i| i.name == new_team.name)
            {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    "team with this name already exists",
                ));
            }

            config.teams.as_mut().unwrap().extend([new_team]);

            let _ = match config::write_config(config) {
                Ok(()) => return Ok(()),
                Err(_error) => return Err(_error),
            };
        }
    }
}
mod remove_cmd {
    use std::io::Error;

    use clap::{arg, App, ArgMatches};

    use crate::{cmd::Cmd, pkg::config};

    pub(crate) fn add_remove_cmd() -> App<'static> {
        return App::new("remove")
            .alias("r")
            .about("Remove the team from the config file")
            .arg(arg!(<TEAM_NAME> "Name the team you're removing"));
    }

    struct RemoveCmd {
        team_name: String,
    }

    pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
        let team_name = sub_matches.value_of("TEAM_NAME").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "team name is not specified",
        ));
        if team_name.is_err() {
            return Err(team_name.err().unwrap());
        }

        Ok(RemoveCmd {
            team_name: team_name.unwrap().to_string(),
        })
    }

    impl<'a> Cmd<'a> for RemoveCmd {
        fn exec(&self) -> Result<(), Error> {
            let mut config = match config::read_config() {
                Ok(c) => c,
                Err(_error) => return Err(_error),
            };

            println!("Removing {} team", self.team_name);

            //  TODO: It shouldn't look that bad, I hope
            config
                .teams
                .as_mut()
                .unwrap()
                .retain(|t| t.name != self.team_name);

            let _ = match config::write_config(config) {
                Ok(()) => return Ok(()),
                Err(_error) => return Err(_error),
            };
        }
    }
}
mod list_cmd {
    use crate::{cmd::Cmd, pkg::config};
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
            let config = match config::read_config() {
                Ok(c) => c,
                Err(_error) => return Err(_error),
            };

            for team in config.teams.unwrap().iter() {
                println!("{}", team.name);
            }
            Ok(())
        }
    }
}
mod add_project_cmd {
    use crate::{
        cmd::{arg_access, arg_gitlab_token, arg_gitlab_url, arg_project_id, arg_team_name, Cmd},
        pkg::config,
        third_party::{self, gitlab::GitlabActions},
        types::types,
    };
    use clap::{App, ArgMatches};
    use gitlab::Gitlab;
    use std::io::{Error, ErrorKind};

    pub(crate) fn add_add_project_cmd() -> App<'static> {
        return App::new("add-project")
            .alias("ap")
            .about("Remove the team from the config file")
            .arg(arg_team_name())
            .arg(arg_access())
            .arg(arg_project_id())
            .arg(arg_gitlab_token())
            .arg(arg_gitlab_url());
    }
    struct AddProjectCmd {
        team_name: String,
        access_level: String,
        gitlab_project_id: u64,
        gitlab_client: Gitlab,
    }

    pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
        let gitlab_token = sub_matches.value_of("token").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "gitlab token is not specified",
        ));
        if gitlab_token.is_err() {
            return Err(gitlab_token.err().unwrap());
        }
        // Get gitlab url from flags
        let gitlab_url = sub_matches.value_of("url").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "gitlab url is not specified",
        ));
        if gitlab_url.is_err() {
            return Err(gitlab_token.err().unwrap());
        }

        // Connect to gitlab
        let gitlab_client: Gitlab = match Gitlab::new(
            gitlab_url.unwrap().to_string(),
            gitlab_token.unwrap().to_string(),
        ) {
            Ok(g) => g,
            Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
        };

        let gitlab_project_id: u64 = match sub_matches.value_of_t("project-id") {
            Ok(pid) => pid,
            Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
        };

        let access_level = sub_matches.value_of("access").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "team name is not specified",
        ));
        if access_level.is_err() {
            return Err(access_level.err().unwrap());
        }

        let team_name = sub_matches.value_of("team-name").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "team name is not s",
        ));
        if team_name.is_err() {
            return Err(team_name.err().unwrap());
        }

        Ok(AddProjectCmd {
            team_name: team_name.unwrap().to_string(),
            access_level: access_level.unwrap().to_string(),
            gitlab_project_id,
            gitlab_client,
        })
    }

    impl<'a> Cmd<'a> for AddProjectCmd {
        fn exec(&self) -> Result<(), Error> {
            let mut config = match config::read_config() {
                Ok(c) => c,
                Err(_error) => return Err(_error),
            };
            let gitlab = third_party::gitlab::new_gitlab_client(self.gitlab_client.to_owned());
            let project = match gitlab.get_project_data_by_id(self.gitlab_project_id) {
                Ok(p) => p,
                Err(_error) => return Err(_error),
            };

            for team in config.teams.as_mut().unwrap().iter_mut() {
                if team.name == self.team_name {
                    let p = types::Project {
                        access_right: self.access_level.to_string(),
                        id: project.id,
                        name: project.name,
                    };
                    match team.projects.as_mut() {
                        Some(v) => {
                            if v.iter().any(|i| i.id == p.id) {
                                return Err(Error::new(
                                    ErrorKind::AlreadyExists,
                                    format!(
                                        "the team '{}' already has an access to this project: '{}'",
                                        team.name, p.name
                                    ),
                                ));
                            }
                            team.projects.as_mut().unwrap().extend([p]);
                        }
                        None => {
                            team.projects = Some(vec![p]);
                        }
                    }
                    break;
                }
            }
            let _ = match config::write_config(config) {
                Ok(()) => return Ok(()),
                Err(_error) => return Err(_error),
            };
        }
    }
}
mod remove_project_cmd {
    use crate::{
        cmd::{arg_project_id, arg_team_name, Cmd},
        pkg::config,
        types::types::{Project},
    };
    use clap::{App, ArgMatches};

    use std::io::{Error, ErrorKind};

    pub(crate) fn add_remove_project_cmd() -> App<'static> {
        return App::new("remove-project")
            .alias("rp")
            .about("Remove a Gitlab project from the team")
            .arg(arg_team_name())
            .arg(arg_project_id());
    }
    struct RemoveProjectCmd {
        team_name: String,
        gitlab_project_id: u64,
    }

    pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
        let gitlab_project_id: u64 = match sub_matches.value_of_t("project-id") {
            Ok(pid) => pid,
            Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
        };

        let team_name = sub_matches.value_of("team-name").ok_or(Error::new(
            std::io::ErrorKind::PermissionDenied,
            "team name is not s",
        ));
        if team_name.is_err() {
            return Err(team_name.err().unwrap());
        }

        Ok(RemoveProjectCmd {
            team_name: team_name.unwrap().to_string(),
            gitlab_project_id,
        })
    }

    impl<'a> Cmd<'a> for RemoveProjectCmd {
        fn exec(&self) -> Result<(), Error> {
            let mut config = match config::read_config() {
                Ok(c) => c,
                Err(_error) => return Err(_error),
            };

            for team in config.teams.as_mut().unwrap().iter_mut() {
                if team.name == self.team_name {
                    let project;
                    match team.projects.as_mut() {
                        Some(v) => {
                            for (_, p) in v.iter().enumerate() {
                                if self.gitlab_project_id == p.id {
                                    project = Project {
                                        name: p.name.to_string(),
                                        id: p.id,
                                        access_right: p.access_right.to_string(),
                                    };
                                    println!("removing {} from {}", project.name, self.team_name);
                                    team.projects
                                        .as_mut()
                                        .unwrap()
                                        .retain(|i| i.id != project.id);

                                    break;
                                }
                            }
                        }
                        None => {
                            return Err(Error::new(
                                ErrorKind::NotFound,
                                "there is no projects in this namespace",
                            ))
                        }
                    };
                }
            }

            let _ = match config::write_config(config) {
                Ok(()) => return Ok(()),
                Err(_error) => return Err(_error),
            };
        }
    }
}
