use crate::args::access_level::ArgAccess;
use crate::args::file_name::ArgFileName;
use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::project_id::ArgProjectId;
use crate::args::team_name::ArgTeamName;
use crate::args::Args;
use crate::cmd::Cmd;
use crate::gitlab::GitlabClient;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::project::Project;
use crate::{gitlab::GitlabActions, types::v1::access_level::AccessLevel};
use clap::{ArgMatches, Command};
use gitlab::Gitlab;
use std::io::{Error, ErrorKind};

pub(crate) fn add_add_project_cmd() -> Command<'static> {
    return Command::new("add-project")
        .alias("ap")
        .about("Remove the team from the config file")
        .arg(ArgTeamName::add())
        .arg(ArgAccess::add())
        .arg(ArgProjectId::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgFileName::add())
        .arg(ArgGitlabUrl::add());
}
struct AddProjectCmd {
    file_name: String,
    team_name: String,
    access_level: AccessLevel,
    gitlab_project_id: u64,
    gitlab_client: Gitlab,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let gitlab_token = match ArgGitlabToken::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let gitlab_url = match ArgGitlabUrl::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    // Connect to gitlab
    let gitlab_client: Gitlab = match Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string())
    {
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };

    let gitlab_project_id: u64 = match ArgProjectId::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let access_level = match ArgAccess::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(e) => return Err(e),
    };

    let team_name = match ArgTeamName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(AddProjectCmd {
        file_name,
        team_name,
        access_level,
        gitlab_project_id,
        gitlab_client,
    })
}

impl<'a> Cmd<'a> for AddProjectCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        let gitlab = GitlabClient::new(self.gitlab_client.to_owned());
        // let project = match gitlab.projects.get(self.gitlab_project_id) {
        let project = match gitlab.get_project_data_by_id(self.gitlab_project_id) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        for team in config_file.config.teams.iter_mut() {
            if team.name == self.team_name {
                let p = Project {
                    name: project.name.to_string(),
                    id: project.id,
                    access_level: self.access_level,
                };
                if team.projects.iter().any(|i| i.id == p.id) {
                    return Err(Error::new(
                        ErrorKind::AlreadyExists,
                        format!(
                            "The team '{}' already has an access to this project: '{}'",
                            team.name, p.name
                        ),
                    ));
                }
                team.projects.extend([p]);
                match config_file.write(self.file_name.clone()) {
                    Ok(()) => {
                        OutMessage::message_info_clean(
                            format!(
                                "The project {} is added to the team {}",
                                project.name, self.team_name
                            )
                            .as_str(),
                        );
                        return Ok(());
                    }
                    Err(err) => return Err(err),
                };
            }
        }
        let error_message = format!("The team with this name can't be found: {}", self.team_name);
        OutMessage::message_error(error_message.as_str());
        return Err(Error::new(ErrorKind::NotFound, error_message));
    }
}

