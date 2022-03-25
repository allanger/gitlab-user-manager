use std::{
    io::{Error, ErrorKind, Result},
    vec,
};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::{
    args::{
        file_name::ArgFileName, gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl,
        group_list::ArgGroupList, Args,
    },
    cmd::CmdOld,
    gitlab::{
        shared_groups, shared_projects, GitlabActions, GitlabClient, GitlabClientApi, Group,
        Project,
    },
    output::out_message::OutMessage,
    service::init::InitService,
    types::v1::{
        self, access_level::AccessLevel, config_file::ConfigFile, namespace::Namespace, user,
    },
};

use super::Cmd;

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_init_cmd() -> Command<'static> {
    return Command::new("init")
        .about("Create a default yaml file in the current directory")
        .arg(ArgFileName::add())
        .arg(ArgGroupList::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add());
}

pub(crate) struct InitCmd {
    file_name: String,
    group_list: Vec<u64>,
    gitlab_url: String,
    gitlab_token: String,
}

// TODO: It's not actually implemented yet
impl Cmd for InitCmd {
    type CmdType = InitCmd;
    fn add() -> Command<'static> {
        Command::new("init")
            .about("Create a default yaml file in the current directory")
            .arg(ArgFileName::add())
            .arg(ArgGroupList::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
    }

    fn prepare(sub_matches: &'_ ArgMatches) -> Result<InitCmd> {
        Ok(InitCmd {
            file_name: ArgFileName::parse(sub_matches)?.value(),
            group_list: ArgGroupList::parse(sub_matches)?.value().to_vec(),
            gitlab_url: ArgGitlabToken::parse(sub_matches)?.value(),
            gitlab_token: ArgGitlabUrl::parse(sub_matches)?.value(),
        })
    }

    fn exec(&self) -> Result<()> {
        InitService::new()
            .parse_groups(self.group_list.clone())
            .save(self.file_name.clone())
    }
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>> {
    let file_name = ArgFileName::parse(sub_matches)?.value();
    let group_list = ArgGroupList::parse(sub_matches)?.value().to_vec();
    let gitlab_token = ArgGitlabToken::parse(sub_matches)?.value();
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?.value();
    Ok(InitCmd {
        file_name,
        group_list,
        gitlab_url,
        gitlab_token,
    })
}

impl<'a> CmdOld<'a> for InitCmd {
    fn exec(&self) -> Result<()> {
        let mut config_file: ConfigFile = Default::default();

        if self.group_list.len() > 0 {
            // Prepare gitlab client
            let gitlab_client: Gitlab =
                match Gitlab::new(self.gitlab_url.to_string(), self.gitlab_token.to_string()) {
                    Ok(g) => g,
                    Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
                };
            let gitlab = GitlabClient::new(gitlab_client.to_owned());
            // Scrap groups
            let mut groups: Vec<Group> = Vec::new();
            OutMessage::message_info_with_alias("Scrapping groups");
            for i in self.group_list.iter() {
                let group = match gitlab.get_group_data_by_id(*i) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                groups.extend(vec![group.clone()]);
                groups.extend(gitlab.get_subgroups(group.name.clone(), *i));
            }
            OutMessage::message_info_with_alias(format!("Got {} groups", groups.len()).as_str());
            // Scrap projects
            let mut projects: Vec<Project> = Vec::new();
            for i in groups.iter() {
                projects.extend(gitlab.get_projects(i.name.clone(), i.id));
            }
            OutMessage::message_info_with_alias(
                format!("Got {} projects", projects.len()).as_str(),
            );

            for g in groups.iter() {
                match shared_groups::SharedWithGroups::get(g.id, gitlab.get_client()) {
                    Ok(group) => {
                        for ns in group.iter() {
                            let item = Namespace {
                                name: g.name.clone(),
                                access_level: AccessLevel::from_gitlab_access_level(
                                    ns.group_access_level,
                                ),
                                id: g.id,
                                url: g.web_url.clone(),
                            };
                            let mut found = false;
                            for group in config_file.config.groups.iter_mut() {
                                if ns.group_id == group.id {
                                    found = true;
                                    group.namespaces.push(item.clone());
                                }
                            }
                            if !found {
                                let group_entry = v1::group::Group {
                                    name: ns.group_name.clone(),
                                    id: ns.group_id,
                                    projects: Default::default(),
                                    namespaces: vec![item],
                                };
                                config_file.config.groups.push(group_entry);
                            }
                        }
                    }
                    Err(_) => {
                        OutMessage::message_info_clean("This group is not shared");
                    }
                };
                // Add user if doesn't exist or add group to user if exists
                let groups_users = gitlab.get_group_members(g.name.to_string(), g.id);
                for member in groups_users.iter() {
                    let mut found = false;
                    for u in config_file.config.users.iter_mut() {
                        if u.id == member.id {
                            found = true;
                            u.namespaces.push(g.to_gum_group(member.clone()).unwrap());
                            break;
                        }
                    }
                    if !found {
                        config_file.config.users.push(user::User {
                            id: member.id,
                            name: member.name.clone(),
                            teams: Default::default(),
                            projects: Default::default(),
                            namespaces: vec![g.to_gum_group(member.clone()).unwrap()],
                        });
                    }
                }
            }

            for p in projects.iter() {
                // Add user if doesn't exist or add group to user if exists
                match shared_projects::SharedWithGroups::get(p.id, gitlab.get_client()) {
                    Ok(group) => {
                        for ns in group.iter() {
                            let item = v1::project::Project {
                                id: p.id,
                                name: p.name.clone(),
                                access_level: AccessLevel::from_gitlab_access_level(
                                    ns.group_access_level,
                                ),
                            };
                            let mut found = false;
                            for group in config_file.config.groups.iter_mut() {
                                if ns.group_id == group.id {
                                    found = true;
                                    group.projects.push(item.clone());
                                }
                            }
                            if !found {
                                let group_entry = v1::group::Group {
                                    name: ns.group_name.clone(),
                                    id: ns.group_id,
                                    namespaces: Default::default(),
                                    projects: vec![item],
                                };
                                config_file.config.groups.push(group_entry);
                            }
                        }
                    }
                    Err(_) => {
                        OutMessage::message_info_clean("This project is not shared");
                    }
                };

                let projects_users = gitlab.get_project_members(p.name.to_string(), p.id);
                for member in projects_users.iter() {
                    let mut found = false;
                    for u in config_file.config.users.iter_mut() {
                        if u.id == member.id {
                            found = true;
                            u.projects.push(p.to_gum_project(member.clone()).unwrap());
                            break;
                        }
                    }
                    if !found {
                        config_file.config.users.push(user::User {
                            id: member.id,
                            name: member.name.clone(),
                            projects: vec![p.to_gum_project(member.clone()).unwrap()],
                            teams: Default::default(),
                            namespaces: Default::default(),
                        });
                    }
                }
            }
        }
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.file_name.clone())
        {
            Ok(file) => file,
            Err(err) => {
                return match err.kind() {
                    ErrorKind::AlreadyExists => {
                        return Err(Error::new(
                            err.kind(),
                            "config file already exists in specified directory",
                        ))
                    }
                    _ => Err(Error::new(ErrorKind::AlreadyExists, err)),
                }
            }
        };

        match config_file.write(self.file_name.clone()) {
            Ok(_) => {
                OutMessage::message_empty(
                    format!(
                        "Config file is generated, check it out\n   $ cat {}",
                        self.file_name.clone()
                    )
                    .as_str(),
                );
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    }
}
