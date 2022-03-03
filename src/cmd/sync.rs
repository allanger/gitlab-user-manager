use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind};
use std::result::Result;

use clap::{ArgMatches, Command};
use gitlab::Gitlab;
use serde::{Deserialize, Serialize};

use crate::args::dry_run::ArgDryRun;
use crate::args::file_name::ArgFileName;
use crate::args::gitlab_token::ArgGitlabToken;
use crate::args::gitlab_url::ArgGitlabUrl;
use crate::args::Args;

use crate::args::state_destination::ArgStateDestination;
use crate::args::state_source::ArgStateSource;
use crate::args::write_state::ArgWriteState;
use crate::output::OutMessage;
use crate::types::v1::access_level::AccessLevel;
use crate::types::v1::config_file::ConfigFile;
use crate::{cmd::Cmd, types::v1::state};

use self::sync_cmd::{apply, compare_states, configure_groups, configure_projects};

pub(crate) fn add_sync_cmd() -> Command<'static> {
    Command::new("sync")
        .about("Sync your config file with GitLab and generate the state file")
        .arg(ArgDryRun::add())
        .arg(ArgGitlabToken::add())
        .arg(ArgGitlabUrl::add())
        .arg(ArgFileName::add())
        .arg(ArgStateDestination::add())
        .arg(ArgStateSource::add())
        .arg(ArgWriteState::add())
}

pub(crate) struct SyncCmd {
    dry_run: bool,
    gitlab_client: Gitlab,
    file_name: String,
    write_state: bool,
    state_destination: String,
    state_source: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let dry_run: bool = ArgDryRun::parse(sub_matches).unwrap().value();
    let write_state: bool = ArgWriteState::parse(sub_matches).unwrap().value();

    let gitlab_token = match ArgGitlabToken::parse(sub_matches) {
        Ok(v) => v.value(),
        Err(err) => return Err(err),
    };
    let gitlab_url = match ArgGitlabUrl::parse(sub_matches) {
        Ok(v) => v.value(),
        Err(err) => return Err(err),
    };

    let gitlab_client = match Gitlab::new(gitlab_url.to_string(), gitlab_token.to_string()) {
        Ok(g) => g,
        Err(err) => return Err(Error::new(ErrorKind::Other, err)),
    };

    let file_name = match ArgFileName::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let state_destination = match ArgStateDestination::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    let state_source = match ArgStateSource::parse(sub_matches) {
        Ok(arg) => arg.value(),
        Err(err) => return Err(err),
    };

    Ok(SyncCmd {
        dry_run,
        gitlab_client,
        file_name,
        state_destination,
        state_source,
        write_state,
    })
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub(crate) struct NewState {
    projects: HashMap<u64, AccessLevel>,
    groups: HashMap<u64, AccessLevel>,
}

impl NewState {
    pub(crate) fn write_to_file(
        state: HashMap<u64, NewState>,
        file_name: String,
    ) -> Result<(), Error> {
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };

        let _ = match serde_json::to_writer(&f, &state) {
            Ok(()) => return Ok(()),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        };
    }
    pub(crate) fn read_from_file(file_name: String) -> Result<HashMap<u64, NewState>, Error> {
        let f = OpenOptions::new().write(true).read(true).open(file_name);

        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        // TODO: Handle serde error
        let d: std::result::Result<HashMap<u64, NewState>, _> = serde_json::from_reader(&f);
        match d {
            Ok(r) => return Ok(r),
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, err.to_string()));
            }
        };
    }
}

impl<'a> Cmd<'a> for SyncCmd {
    fn exec(&self) -> Result<(), Error> {
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        // Read old state
        let mut old_state: HashMap<u64, NewState> = HashMap::new();
        if self.state_source != "" {
            OutMessage::message_info_with_alias(
                format!("I will try to use this file: {}", self.state_source.clone()).as_str(),
            );
            old_state = NewState::read_from_file(self.state_source.clone())?
        } else {
            if config_file.state.as_str() != "~" {
                OutMessage::message_info_with_alias("State is found");
                old_state = match serde_json::from_str(config_file.state.as_str()) {
                    Ok(state) => state,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
                };
            } else {
                OutMessage::message_info_with_alias(
                    "State is not found, I will generate a new one",
                );
            }
        }
        let mut new_state: HashMap<u64, NewState> = HashMap::new();
        for u in config_file.config.users.iter().clone() {
            new_state.insert(
                u.id,
                NewState {
                    projects: configure_projects(u, config_file.config.clone()),
                    groups: configure_groups(u, config_file.config.clone()),
                },
            );
        }

        let actions = compare_states(old_state.clone(), new_state);
        let state: String = match apply(actions, &self.gitlab_client, &mut old_state, self.dry_run)
        {
            Ok(_) => serde_json::to_string(&old_state).unwrap(),
            Err(err) => {
                OutMessage::message_error(
                    format!(
                        "This error happened while I was applying:\n {}\nI'll save the intermediate state",
                        err
                    )
                    .as_str(),
                );
                serde_json::to_string(&old_state).unwrap()
            }
        };
        if !self.dry_run {
            config_file.state = state;
        }
        if self.write_state {
            NewState::write_to_file(old_state, self.state_destination.clone());
        }

        match config_file.write(self.file_name.clone()) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        }
    }
}

mod sync_cmd {
    #[derive(Debug, Clone)]
    pub(crate) struct Actions {
        user_id: u64,
        entity_id: u64,
        entity_type: EntityType,
        access: AccessLevel,
        action: Action,
    }

    use std::collections::HashMap;
    use std::io::Error;

    use ::gitlab::Gitlab;

    use crate::gitlab::{GitlabActions, GitlabClient, Group};
    use crate::output::{OutMessage, OutSpinner, OutSum};

    use crate::types::v1::{
        access_level::AccessLevel, config::Config, ownership::Ownership, project::Project,
        state::State, user::User,
    };

    use super::NewState;

    pub(crate) fn apply(
        actions: Vec<Actions>,
        gitlab_client: &Gitlab,
        state: &mut HashMap<u64, NewState>,
        dry: bool,
    ) -> Result<(), Error> {
        for a in actions.iter() {
            let gitlab = GitlabClient::new(gitlab_client.to_owned());
            let username = match gitlab.get_user_data_by_id(a.user_id) {
                Ok(r) => r,
                Err(err) => return Err(err),
            };
            match a.entity_type {
                EntityType::PROJECT => {
                    let project = match gitlab.get_project_data_by_id(a.entity_id) {
                        Ok(r) => r,
                        Err(err) => return Err(err),
                    };
                    match a.action {
                        Action::CREATE => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Adding {} to {} as {}",
                                    username.name, project.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match gitlab.add_user_to_project(a.user_id, a.entity_id, a.access) {
                                    Err(err) => {
                                        spinner.spinner_failure(err.to_string());
                                        return Err(err);
                                    }
                                    Ok(msg) => {
                                        spinner.spinner_success(msg.to_string());
                                    }
                                }
                            } else {
                                spinner.spinner_close();
                            }
                            if !state.contains_key(&a.user_id) {
                                state.insert(a.user_id, NewState::default());
                            };
                            if let Some(x) = state.get_mut(&a.user_id) {
                                x.projects.insert(a.entity_id, a.access);
                            }
                        }
                        Action::DELETE => {
                            let spinner = OutSpinner::spinner_start(
                                format!("Removing {} from {}", username.name, project.name)
                                    .to_string(),
                            );
                            if !dry {
                                match gitlab.remove_user_from_project(a.user_id, a.entity_id) {
                                    Err(err) => {
                                        spinner.spinner_failure(err.to_string());
                                        return Err(err);
                                    }
                                    Ok(msg) => {
                                        spinner.spinner_success(msg.to_string());
                                    }
                                }
                            } else {
                                spinner.spinner_close();
                            }
                            if let Some(x) = state.get_mut(&a.user_id) {
                                x.projects
                                    .remove(&a.entity_id)
                                    .ok_or_else(|| {
                                        OutMessage::message_error(
                                            format!(
                                                "Project {} can't be found in state",
                                                a.entity_id
                                            )
                                            .as_str(),
                                        )
                                    })
                                    .unwrap();
                            }
                        }
                        Action::UPDATE => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Updating {} in {} to {}",
                                    username.name, project.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match gitlab.edit_user_in_project(a.user_id, a.entity_id, a.access)
                                {
                                    Err(err) => {
                                        spinner.spinner_failure(err.to_string());
                                        return Err(err);
                                    }
                                    Ok(msg) => {
                                        spinner.spinner_success(msg.to_string());
                                    }
                                }
                            } else {
                                spinner.spinner_close();
                            }
                            if let Some(x) = state.get_mut(&a.user_id) {
                                x.projects.insert(a.entity_id, a.access);
                            }
                        }
                    }
                }
                EntityType::GROUP => {
                    let group = match gitlab.get_group_data_by_id(a.entity_id) {
                        Ok(r) => r,
                        Err(err) => return Err(err),
                    };
                    match a.action {
                        Action::CREATE => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Adding {} to {} as {}",
                                    username.name, group.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match gitlab.add_user_to_group(a.user_id, a.entity_id, a.access) {
                                    Err(err) => {
                                        spinner.spinner_failure(err.to_string());
                                        return Err(err);
                                    }
                                    Ok(msg) => {
                                        spinner.spinner_success(msg.to_string());
                                    }
                                }
                            } else {
                                spinner.spinner_close();
                            }
                            if !state.contains_key(&a.user_id) {
                                state.insert(a.user_id, NewState::default());
                            };
                            if let Some(x) = state.get_mut(&a.user_id) {
                                x.groups.insert(a.entity_id, a.access);
                            }
                        }
                        Action::DELETE => {
                            let spinner = OutSpinner::spinner_start(
                                format!("Removing {} from {}", username.name, group.name)
                                    .to_string(),
                            );
                            if !dry {
                                match gitlab.remove_user_from_group(a.user_id, a.entity_id) {
                                    Err(err) => {
                                        spinner.spinner_failure(err.to_string());
                                        return Err(err);
                                    }
                                    Ok(msg) => {
                                        spinner.spinner_success(msg.to_string());
                                    }
                                }
                            } else {
                                spinner.spinner_close();
                            };
                            if let Some(x) = state.get_mut(&a.user_id) {
                                x.groups
                                    .remove(&a.entity_id)
                                    .ok_or_else(|| {
                                        OutMessage::message_error(
                                            format!(
                                                "Project {} can't be found in state",
                                                a.entity_id
                                            )
                                            .as_str(),
                                        )
                                    })
                                    .unwrap();
                            }
                        }
                        Action::UPDATE => {
                            OutSum::sum_failure("Groups can't be updated yet, because only owner access is allowed for groups");
                        }
                    }
                }
            }
        }
        OutMessage::message_info_with_alias("You are synchronized, now but not forever");
        Ok(())
    }

    pub(crate) fn configure_groups(u: &User, c: Config) -> HashMap<u64, AccessLevel> {
        let mut groups_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut groups: Vec<Ownership> = u.ownerships.clone();
        for g in groups.iter() {
            let mut group: HashMap<u64, AccessLevel> = HashMap::new();
            groups_map.insert(g.id, AccessLevel::Owner);
        }
        return groups_map;
    }

    pub(crate) fn configure_projects(u: &User, c: Config) -> HashMap<u64, AccessLevel> {
        let mut projects_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut projects: Vec<Project> = u.projects.clone();
        for t in c.teams.iter() {
            if u.teams.contains(&t.name.to_string()) || t.name == "default" {
                projects.extend(t.projects.clone());
            }
        }

        let mut keys: HashMap<u64, Project> = HashMap::new();
        for p in projects.iter() {
            if !keys.contains_key(&p.id) {
                keys.insert(p.id, p.clone());
            } else {
                keys.insert(p.id, higher_access(p, keys.get(&p.id).unwrap()));
            }
        }
        projects.clear();
        for p in keys.iter() {
            projects_map.insert(p.1.id, p.1.access_level);
        }
        return projects_map;
    }

    pub(crate) fn configure_projects_old<'a>(u: &User, c: Config) -> Vec<Project> {
        // Get projects from user and from teams to which this user belongs
        let mut projects: Vec<Project> = u.projects.clone();
        for t in c.teams.iter() {
            if u.teams.contains(&t.name.to_string()) || t.name == "default" {
                projects.extend(t.projects.clone());
            }
        }
        let mut keys: HashMap<u64, Project> = HashMap::new();
        for p in projects.iter() {
            if !keys.contains_key(&p.id) {
                keys.insert(p.id, p.clone());
            } else {
                keys.insert(p.id, higher_access(p, keys.get(&p.id).unwrap()));
            }
        }
        projects.clear();
        for p in keys.iter() {
            projects.extend([p.1.clone()]);
        }

        projects
    }

    fn higher_access<'a>(project1: &'a Project, project2: &'a Project) -> Project {
        let access_level: AccessLevel;

        if project1.access_level == AccessLevel::Maintainer
            || project2.access_level == AccessLevel::Maintainer
        {
            access_level = AccessLevel::Maintainer;
        } else if project1.access_level == AccessLevel::Developer
            || project2.access_level == AccessLevel::Developer
        {
            access_level = AccessLevel::Developer;
        } else if project1.access_level == AccessLevel::Reporter
            || project2.access_level == AccessLevel::Reporter
        {
            access_level = AccessLevel::Reporter;
        } else {
            access_level = AccessLevel::Guest;
        }

        let p = Project {
            name: project1.name.clone(),
            id: project1.id.clone(),
            access_level,
        };
        p
    }

    #[derive(Debug, Clone)]
    enum EntityType {
        PROJECT,
        GROUP,
    }
    #[derive(Debug, Clone)]
    enum Action {
        CREATE,
        DELETE,
        UPDATE,
    }

    pub(crate) fn compare_states(
        mut old_state: HashMap<u64, NewState>,
        new_state: HashMap<u64, NewState>,
    ) -> Vec<Actions> {
        let mut actions: Vec<Actions> = Vec::new();

        for (id, state) in new_state.clone().iter() {
            if old_state.contains_key(id) {
                compare_projects(
                    old_state[id].projects.clone(),
                    state.projects.clone(),
                    &mut actions,
                    *id,
                );
                compare_ownerships(
                    old_state[id].groups.clone(),
                    state.groups.clone(),
                    &mut actions,
                    *id,
                );
                old_state.remove(id);
            } else {
                for (pid, access) in state.projects.iter() {
                    actions.extend([Actions {
                        user_id: id.clone(),
                        entity_id: pid.clone(),
                        entity_type: EntityType::PROJECT,
                        access: access.clone(),
                        action: Action::CREATE,
                    }])
                }
                for (gid, access) in state.groups.iter() {
                    actions.extend([Actions {
                        user_id: id.clone(),
                        entity_id: gid.clone(),
                        entity_type: EntityType::GROUP,
                        access: access.clone(),
                        action: Action::CREATE,
                    }])
                }
            }
        }
        for (id, state) in old_state.iter() {
            for (pid, access) in state.projects.iter() {
                actions.extend([Actions {
                    user_id: id.clone(),
                    entity_id: pid.clone(),
                    entity_type: EntityType::PROJECT,
                    access: access.clone(),
                    action: Action::DELETE,
                }])
            }
            for (gid, access) in state.groups.iter() {
                actions.extend([Actions {
                    user_id: id.clone(),
                    entity_id: gid.clone(),
                    entity_type: EntityType::GROUP,
                    access: access.clone(),
                    action: Action::DELETE,
                }])
            }
        }

        actions
    }

    fn compare_ownerships(
        mut old_state: HashMap<u64, AccessLevel>,
        new_state: HashMap<u64, AccessLevel>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        for (id, access) in new_state.iter() {
            if old_state.contains_key(id) {
                if old_state[id] != new_state[id] {
                    actions.extend([Actions {
                        user_id,
                        entity_id: id.clone(),
                        entity_type: EntityType::GROUP,
                        access: access.clone(),
                        action: Action::UPDATE,
                    }]);
                }
                old_state.remove(id);
            } else {
                actions.extend([Actions {
                    user_id,
                    entity_id: id.clone(),
                    entity_type: EntityType::GROUP,
                    access: access.clone(),
                    action: Action::CREATE,
                }])
            }
        }
        for (id, access) in old_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: id.clone(),
                entity_type: EntityType::GROUP,
                access: access.clone(),
                action: Action::DELETE,
            }])
        }
    }

    fn compare_projects(
        mut old_state: HashMap<u64, AccessLevel>,
        mut new_state: HashMap<u64, AccessLevel>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        for (id, access) in new_state.iter() {
            if old_state.contains_key(id) {
                if old_state[id] != new_state[id] {
                    actions.extend([Actions {
                        user_id,
                        entity_id: id.clone(),
                        entity_type: EntityType::PROJECT,
                        access: access.clone(),
                        action: Action::UPDATE,
                    }]);
                }
                old_state.remove(id);
            } else {
                actions.extend([Actions {
                    user_id,
                    entity_id: id.clone(),
                    entity_type: EntityType::PROJECT,
                    access: access.clone(),
                    action: Action::CREATE,
                }])
            }
        }
        for (id, access) in old_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: id.clone(),
                entity_type: EntityType::PROJECT,
                access: access.clone(),
                action: Action::DELETE,
            }])
        }
    }
}
