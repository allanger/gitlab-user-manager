use std::io::{Error, ErrorKind};

use clap::{App, Arg, ArgMatches};
use gitlab::Gitlab;

use crate::{cmd::Cmd, files};

use super::{arg_gitlab_token, arg_gitlab_url};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_sync_cmd() -> App<'static> {
    let dry_run = Arg::new("dry_run")
        .short('d')
        .takes_value(true)
        .value_name("DRY_RUN")
        .default_value("false")
        .help("Use if you wanna see what's gonna happen without applying new configuration");
    return App::new("sync")
        .about("Sync your config file with GitLab and generate the state file")
        .arg(dry_run)
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

pub(crate) struct SyncCmd {
    dry_run: bool,
    gitlab_client: Option<Gitlab>,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let dry_run: bool = match sub_matches.value_of_t("dry-run") {
        Ok(dr) => dr,
        Err(_error) => return Err(Error::new(ErrorKind::InvalidInput, _error.to_string())),
    };
    let gitlab_client: Option<Gitlab>;
    if dry_run {
        gitlab_client = None
    } else {
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
        gitlab_client = match Gitlab::new(
            gitlab_url.unwrap().to_string(),
            gitlab_token.unwrap().to_string(),
        ) {
            Ok(g) => Some(g),
            Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
        };
    }
    Ok(SyncCmd {
        dry_run,
        gitlab_client,
    })
}

impl<'a> Cmd<'a> for SyncCmd {
    fn exec(&self) -> Result<(), Error> {
        let config = match files::read_config() {
            Ok(c) => c,
            Err(_error) => return Err(_error),
        };

        for u in config.users.iter() {}
        Ok(())
    }
}

mod sync_cmd {
    use crate::types::{config::Config, project::{Project, self}, user::User};

    pub(crate) fn configure_projects(u: &User, c: &Config) -> Vec<Project> {
        /*
        var projects []models.Project
        for _, t := range c.Teams {
            if contains(u.Teams, t.Name) {
                projects = append(projects, t.Projects...)
            } else if t.Name == "default" {
                projects = append(projects, t.Projects...)
            }
        }
        projects = append(projects, u.Projects...)
        return projects

            */
        let mut projects: Vec<Project> = Vec::new();
        for t in c.teams.iter(){
            if u.teams.contains(&t.name.to_string()) {
                projects.extend(t.projects);
            }
        }

        return vec![Project {
            ..Default::default()
        }];
    }
}
