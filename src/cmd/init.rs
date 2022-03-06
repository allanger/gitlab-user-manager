use std::{
    io::{Error, ErrorKind},
    vec,
};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::{
    args::{
        file_name::ArgFileName, gitlab_token::ArgGitlabToken, gitlab_url::ArgGitlabUrl,
        group_list::ArgGroupList, Args,
    },
    cmd::Cmd,
    gitlab::{CustomMember, GitlabActions, GitlabClient, Group, Project, User},
    output::out_message::OutMessage,
    types::v1::{config::Config, config_file::ConfigFile, team::Team, user},
};

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

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };
    let group_list = match ArgGroupList::parse(sub_matches) {
        Ok(arg) => arg.value().to_vec(),
        Err(err) => return Err(err),
    };
    let gitlab_token = match ArgGitlabToken::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let gitlab_url = match ArgGitlabUrl::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(InitCmd {
        file_name,
        group_list,
        gitlab_url,
        gitlab_token,
    })
}

impl<'a> Cmd<'a> for InitCmd {
    fn exec(&self) -> Result<(), Error> {
        let config_file = ConfigFile::default();
        // let mut config_file:ConfigFile = Default::default();
        // if self.group_list.len() > 0 {
            // let gitlab_client: Gitlab =
                // match Gitlab::new(self.gitlab_url.to_string(), self.gitlab_token.to_string()) {
                    // Ok(g) => g,
                    // Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
                // };
            // let gitlab = GitlabClient::new(gitlab_client.to_owned());
            // let mut sub: Vec<Group> = Vec::new();
            // OutMessage::message_info_with_alias("Scrapping groups");
            // for i in self.group_list.iter() {
                // let group = match gitlab.get_group_data_by_id(*i) {
                    // Ok(p) => p,
                    // Err(err) => return Err(err),
                // };
                // sub.extend(vec![group.clone()]);
                // sub.extend(gitlab.get_subgroups(group.name.clone(), *i));
            // }
            // OutMessage::message_info_with_alias(format!("Got {} groups", sub.len() + 1).as_str());
            // let mut projects: Vec<Project> = Vec::new();
            // for i in sub.iter() {
                // projects.extend(gitlab.get_projects(i.name.clone(), i.id));
            // }
            // OutMessage::message_info_with_alias(
                // format!("Got {} projects", projects.len() + 1).as_str(),
            // );
// 
            // println!("1");
            // let mut groups_users: Vec<CustomMember> = Vec::new();
            // println!("2");
// 
            // for g in sub.iter() {
                // Add user if doesn't exist or add group to user if exists
                // println!("3");
                // groups_users.extend(gitlab.get_group_members(g.name.to_string(), g.id));
                // println!("4");
                // for member in groups_users.iter() {
                    // let mut found = false;
                    // for u in config_file.config.users.iter_mut() {
                        // if u.id == member.id {
                            // found = true;
                            // u.groups.push(g.to_gum_group(member.clone()).unwrap());
                        // }
                    // }
                    // if found {
                        // config_file.config.users.push(user::User {
                            // id: member.id,
                            // name: member.name.clone(),
                            // teams: Default::default(),
                            // projects: Default::default(),
                            // groups: vec![g.to_gum_group(member.clone()).unwrap()],
                        // });
                    // }
                // }
            // }
            // let mut projects_users: Vec<CustomMember> = Vec::new();
            // for p in projects.iter() {
                // projects_users.extend(gitlab.get_project_members(p.name.to_string(), p.id));
            // }
            // for p in projects.iter() {
                // Add user if doesn't exist or add group to user if exists
                // projects_users.extend(gitlab.get_project_members(p.name.to_string(), p.id));
                // for member in groups_users.iter() {
                    // let mut found = false;
                    // for u in config_file.config.users.iter_mut() {
                        // if u.id == member.id {
                            // found = true;
                            // u.projects.push(p.to_gum_project(member.clone()).unwrap());
                        // }
                    // }
                    // if found {
                        // config_file.config.users.push(user::User {
                            // id: member.id,
                            // name: member.name.clone(),
                            // teams: Default::default(),
                            // projects: vec![p.to_gum_project(member.clone()).unwrap()],
                            // groups: Default::default(),
                        // });
                    // }
                // }
            // }
        // }

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
                        "Config file is generated, check it out\n $ cat {}",
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
