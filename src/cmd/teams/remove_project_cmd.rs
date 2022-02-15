use crate::{
    cmd::{arg_project_id, arg_team_name, Cmd},
    files,
    types::project::Project,
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
        let mut config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };
        // TODO: This should be refactored
        for team in config.teams.iter_mut() {
            if team.name == self.team_name {
                let project;
                for (i, p) in team.projects.iter().enumerate() {
                    if self.gitlab_project_id == p.id {
                        project = Project {
                            name: p.name.to_string(),
                            id: p.id,
                            ..Default::default()
                        };
                        println!("removing {} from {}", project.name, self.team_name);
                        team.projects.remove(i);
                        break;
                    }
                }
            }
        }

        let _ = match files::write_config(config) {
            Ok(()) => return Ok(()),
            Err(_error) => return Err(_error),
        };
    }
}
