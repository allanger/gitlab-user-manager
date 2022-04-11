use crate::Cmd;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

use clap::{ArgMatches, Command};
use gitlab::Gitlab;

use crate::args::{
    ArgDryRun, ArgFileName, ArgGitlabToken, ArgGitlabUrl, ArgStateDestination, ArgStateSource,
    ArgWriteState, Args,
};

use crate::cmd::CmdOld;
use crate::output::out_message::OutMessage;
use crate::types::v1::config_file::ConfigFile;
use crate::types::v1::state::{AccessUnit, EntityType};

use self::sync_cmd::{
    apply, compare_states, configure_groups, configure_projects, gr_configure_groups,
    gr_configure_projects,
};

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
    gitlab_token: String,
    gitlab_url: String,
    file_name: String,
    write_state: bool,
    state_destination: String,
    state_source: String,
}

pub(crate) fn prepare<'a>(sub_matches: &'_ ArgMatches) -> Result<impl CmdOld<'a>> {
    let dry_run: bool = ArgDryRun::parse(sub_matches).unwrap();
    let write_state: bool = ArgWriteState::parse(sub_matches).unwrap();

    let gitlab_token = ArgGitlabToken::parse(sub_matches)?;
    let gitlab_url = ArgGitlabUrl::parse(sub_matches)?;

    let file_name = ArgFileName::parse(sub_matches)?;
    let state_destination = ArgStateDestination::parse(sub_matches)?;
    let state_source = ArgStateSource::parse(sub_matches)?;

    Ok(SyncCmd {
        dry_run,
        file_name,
        gitlab_token,
        gitlab_url,
        state_destination,
        state_source,
        write_state,
    })
}

impl<'a> Cmd for SyncCmd {
    type CmdType = SyncCmd;
    fn add() -> Command<'static> {
        Command::new("sync")
            .about("Sync your config file with GitLab and generate the state file")
            .alias("s")
            .after_help("$ gum sync -f gum-config-example.yaml --dry-run")
            .before_help("Use this command if you want to apply changes in your configuration file to GitLab")
            .arg(ArgDryRun::add())
            .arg(ArgGitlabToken::add())
            .arg(ArgGitlabUrl::add())
            .arg(ArgFileName::add())
            .arg(ArgStateDestination::add())
            .arg(ArgStateSource::add())
            .arg(ArgWriteState::add())
    }
    
    fn prepare(sub_matches: &ArgMatches) -> Result<Self> {
        Ok(SyncCmd {
            dry_run: ArgDryRun::parse(sub_matches)?,
            file_name: ArgFileName::parse(sub_matches)?,
            gitlab_token: ArgGitlabToken::parse(sub_matches)?,
            gitlab_url: ArgGitlabUrl::parse(sub_matches)?,
            state_destination: ArgStateDestination::parse(sub_matches)?,
            state_source: ArgStateSource::parse(sub_matches)?,
            write_state: ArgWriteState::parse(sub_matches)?,
        })
    }
    fn exec(&self) -> Result<()> {
        todo!()
    }
}

impl<'a> CmdOld<'a> for SyncCmd {
    fn exec(&self) -> Result<()> {
        let gitlab_client = Gitlab::new(&self.gitlab_url, &self.gitlab_token)
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let mut config_file = match ConfigFile::read(self.file_name.clone()) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        // Read old state
        let mut old_state: HashMap<u64, AccessUnit> = HashMap::new();
        if !self.state_source.is_empty() {
            OutMessage::message_info_with_alias(
                format!("I will try to use this file: {}", self.state_source.clone()).as_str(),
            );
            old_state = AccessUnit::read_from_file(self.state_source.clone())?
        } else {
            if config_file.state.as_str() == "~" || config_file.state.is_empty() {
                OutMessage::message_info_with_alias(
                    "State is not found, I will generate a new one",
                );
            } else {
                OutMessage::message_info_with_alias("State is found");
                old_state = match serde_json::from_str(config_file.state.as_str()) {
                    Ok(state) => state,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
                };
            }
        }
        let mut new_state: HashMap<u64, AccessUnit> = HashMap::new();
        for u in config_file.config.users.iter().clone() {
            new_state.insert(
                u.id,
                AccessUnit {
                    projects: configure_projects(u, config_file.config.clone()),
                    namespaces: configure_groups(u, config_file.config.clone()),
                    entity: EntityType::User,
                },
            );
        }
        for u in config_file.config.groups.iter().clone() {
            new_state.insert(
                u.id,
                AccessUnit {
                    projects: gr_configure_projects(u),
                    namespaces: gr_configure_groups(u),
                    entity: EntityType::Group,
                },
            );
        }

        let actions = compare_states(old_state.clone(), new_state);
        let state: String = match apply(actions, &gitlab_client, &mut old_state, self.dry_run) {
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
            match AccessUnit::write_to_file(old_state, self.state_destination.clone()) {
                Ok(_) => {
                    let msg = format!(
                        "State is saved, check it out\n $ cat {}",
                        self.state_destination.clone()
                    );
                    OutMessage::message_empty(msg.as_str());
                }
                Err(_) => OutMessage::message_empty("Couldn't save state to file"),
            };
        }

        match config_file.write(self.file_name.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

mod sync_cmd {
    #[derive(Debug, Clone)]
    pub(crate) struct Actions {
        subject_entity_id: u64,
        subject_entity_type: EntityType,
        object_entity_id: u64,
        object_entity_type: ObjectEntityType,
        access: AccessLevel,
        action: Action,
    }

    use std::collections::HashMap;
    use std::io::Error;

    use ::gitlab::Gitlab;

    use crate::gitlab::{GitlabActions, GitlabClient};
    use crate::output::{out_message::OutMessage, out_spinner::OutSpinner};

    use crate::types::v1::group::Group;
    use crate::types::v1::state::EntityType;
    use crate::types::v1::{
        access_level::AccessLevel, config::Config, namespace::Namespace, project::Project,
        state::AccessUnit, user::User,
    };

    pub(crate) fn apply(
        actions: Vec<Actions>,
        gitlab_client: &Gitlab,
        state: &mut HashMap<u64, AccessUnit>,
        dry: bool,
    ) -> Result<(), Error> {
        for a in actions.iter() {
            let gitlab = GitlabClient::new(gitlab_client.to_owned());
            let subject_name = match a.subject_entity_type {
                EntityType::User => match gitlab.get_user_data_by_id(a.subject_entity_id) {
                    Ok(r) => r.name,
                    Err(err) => return Err(err),
                },
                EntityType::Group => match gitlab.get_group_data_by_id(a.subject_entity_id) {
                    Ok(r) => r.name,
                    Err(err) => return Err(err),
                },
            };

            match a.object_entity_type {
                ObjectEntityType::Project => {
                    let project = match gitlab.get_project_data_by_id(a.object_entity_id) {
                        Ok(r) => r,
                        Err(err) => return Err(err),
                    };
                    match a.action {
                        Action::Create => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Adding {} to {} as {}",
                                    subject_name, project.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.add_user_to_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.add_group_to_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            }
                            if !state.contains_key(&a.subject_entity_id) {
                                state.insert(
                                    a.subject_entity_id,
                                    AccessUnit::new_simple(a.subject_entity_type.clone()),
                                );
                            };
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.projects.insert(a.object_entity_id, a.access);
                            }
                        }
                        Action::Delete => {
                            let spinner = OutSpinner::spinner_start(
                                format!("Removing {} from {}", subject_name, project.name)
                                    .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.remove_user_from_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.remove_group_from_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            }
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.projects
                                    .remove(&a.object_entity_id)
                                    .ok_or_else(|| {
                                        OutMessage::message_error(
                                            format!(
                                                "Project {} can't be found in state",
                                                a.object_entity_id
                                            )
                                            .as_str(),
                                        )
                                    })
                                    .unwrap();
                            }
                        }
                        Action::Update => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Updating {} in {} to {}",
                                    subject_name, project.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.edit_user_in_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.remove_group_from_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(_) => {
                                                match gitlab.add_group_to_project(
                                                    a.subject_entity_id,
                                                    a.object_entity_id,
                                                    a.access,
                                                ) {
                                                    Err(err) => {
                                                        spinner.spinner_failure(err.to_string());
                                                        return Err(err);
                                                    }
                                                    Ok(msg) => {
                                                        spinner.spinner_success(msg.to_string());
                                                    }
                                                };
                                            }
                                        };
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            }
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.projects.insert(a.object_entity_id, a.access);
                            }
                        }
                    }
                }
                ObjectEntityType::Group => {
                    let group = match gitlab.get_group_data_by_id(a.object_entity_id) {
                        Ok(r) => r,
                        Err(err) => return Err(err),
                    };
                    match a.action {
                        Action::Create => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Adding {} to {} as {}",
                                    subject_name, group.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.add_user_to_group(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.add_group_to_namespace(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            }
                            if !state.contains_key(&a.subject_entity_id) {
                                state.insert(
                                    a.subject_entity_id,
                                    AccessUnit::new_simple(a.subject_entity_type.clone()),
                                );
                            };
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.namespaces.insert(a.object_entity_id, a.access);
                            }
                        }
                        Action::Delete => {
                            let spinner = OutSpinner::spinner_start(
                                format!("Removing {} from {}", subject_name, group.name)
                                    .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.remove_user_from_group(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.remove_group_from_namespace(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            };
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.namespaces
                                    .remove(&a.object_entity_id)
                                    .ok_or_else(|| {
                                        OutMessage::message_error(
                                            format!(
                                                "Project {} can't be found in state",
                                                a.object_entity_id
                                            )
                                            .as_str(),
                                        )
                                    })
                                    .unwrap();
                            }
                        }
                        Action::Update => {
                            let spinner = OutSpinner::spinner_start(
                                format!(
                                    "Updating {} in {} to {}",
                                    subject_name, group.name, a.access
                                )
                                .to_string(),
                            );
                            if !dry {
                                match a.subject_entity_type {
                                    EntityType::User => {
                                        match gitlab.edit_user_in_group(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                            a.access,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(msg) => {
                                                spinner.spinner_success(msg.to_string());
                                            }
                                        }
                                    }
                                    EntityType::Group => {
                                        match gitlab.remove_group_from_namespace(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(_) => {
                                                match gitlab.add_group_to_namespace(
                                                    a.subject_entity_id,
                                                    a.object_entity_id,
                                                    a.access,
                                                ) {
                                                    Err(err) => {
                                                        spinner.spinner_failure(err.to_string());
                                                        return Err(err);
                                                    }
                                                    Ok(msg) => {
                                                        spinner.spinner_success(msg.to_string());
                                                    }
                                                };
                                            }
                                        };
                                    }
                                };
                            } else {
                                spinner.spinner_close();
                            }
                            if let Some(x) = state.get_mut(&a.subject_entity_id) {
                                x.namespaces.insert(a.object_entity_id, a.access);
                            }
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
        let mut groups: Vec<Namespace> = u.namespaces.clone();
        for t in c.teams.iter() {
            if u.teams.contains(&t.name.to_string()) || t.name == "default" {
                groups.extend(t.namespaces.clone());
            }
        }

        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for g in groups.iter() {
            if !keys.contains_key(&g.id) {
                keys.insert(g.id, g.clone().access_level);
            } else {
                keys.insert(
                    g.id,
                    higher_access(g.access_level, *keys.get(&g.id).unwrap()),
                );
            }
        }
        groups.clear();
        for (k, v) in keys.iter() {
            groups_map.insert(*k, *v);
        }
        groups_map
    }

    pub(crate) fn configure_projects(u: &User, c: Config) -> HashMap<u64, AccessLevel> {
        let mut projects_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut projects: Vec<Project> = u.projects.clone();
        for t in c.teams.iter() {
            if u.teams.contains(&t.name.to_string()) || t.name == "default" {
                projects.extend(t.projects.clone());
            }
        }

        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for p in projects.iter() {
            if !keys.contains_key(&p.id) {
                keys.insert(p.id, p.clone().access_level);
            } else {
                keys.insert(
                    p.id,
                    higher_access(p.access_level, *keys.get(&p.id).unwrap()),
                );
            }
        }
        projects.clear();
        for (k, v) in keys.iter() {
            projects_map.insert(*k, *v);
        }
        return projects_map;
    }

    pub(crate) fn gr_configure_groups(u: &Group) -> HashMap<u64, AccessLevel> {
        let mut groups_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut groups: Vec<Namespace> = u.namespaces.clone();
        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for g in groups.iter() {
            if !keys.contains_key(&g.id) {
                keys.insert(g.id, g.clone().access_level);
            } else {
                keys.insert(
                    g.id,
                    higher_access(g.access_level, *keys.get(&g.id).unwrap()),
                );
            }
        }
        groups.clear();
        for (k, v) in keys.iter() {
            groups_map.insert(*k, *v);
        }
        groups_map
    }
    pub(crate) fn gr_configure_projects(u: &Group) -> HashMap<u64, AccessLevel> {
        let mut projects_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut projects: Vec<Project> = u.projects.clone();
        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for p in projects.iter() {
            if !keys.contains_key(&p.id) {
                keys.insert(p.id, p.clone().access_level);
            } else {
                keys.insert(
                    p.id,
                    higher_access(p.access_level, *keys.get(&p.id).unwrap()),
                );
            }
        }
        projects.clear();
        for (k, v) in keys.iter() {
            projects_map.insert(*k, *v);
        }
        return projects_map;
    }

    fn higher_access(a1: AccessLevel, a2: AccessLevel) -> AccessLevel {
        if a1 == AccessLevel::Maintainer || a2 == AccessLevel::Maintainer {
            AccessLevel::Maintainer
        } else if a1 == AccessLevel::Developer || a2 == AccessLevel::Developer {
            AccessLevel::Developer
        } else if a1 == AccessLevel::Reporter || a2 == AccessLevel::Reporter {
            AccessLevel::Reporter
        } else {
            AccessLevel::Guest
        }
    }

    #[derive(Debug, Clone)]
    enum ObjectEntityType {
        Project,
        Group,
    }
    #[derive(Debug, Clone)]
    enum Action {
        Create,
        Delete,
        Update,
    }

    pub(crate) fn compare_states(
        mut old_state: HashMap<u64, AccessUnit>,
        new_state: HashMap<u64, AccessUnit>,
    ) -> Vec<Actions> {
        let mut actions: Vec<Actions> = Vec::new();

        for (id, state) in new_state.iter() {
            if old_state.contains_key(id) {
                compare_projects(
                    state.entity.clone(),
                    old_state[id].projects.clone(),
                    state.projects.clone(),
                    &mut actions,
                    *id,
                );
                compare_ownerships(
                    state.entity.clone(),
                    old_state[id].namespaces.clone(),
                    state.namespaces.clone(),
                    &mut actions,
                    *id,
                );
                old_state.remove(id);
            } else {
                for (pid, access) in state.projects.iter() {
                    actions.extend([Actions {
                        subject_entity_id: *id,
                        object_entity_id: *pid,
                        object_entity_type: ObjectEntityType::Project,
                        access: *access,
                        action: Action::Create,
                        subject_entity_type: state.entity.clone(),
                    }])
                }
                for (gid, access) in state.namespaces.iter() {
                    actions.extend([Actions {
                        subject_entity_id: *id,
                        object_entity_id: *gid,
                        object_entity_type: ObjectEntityType::Group,
                        access: *access,
                        action: Action::Create,
                        subject_entity_type: state.entity.clone(),
                    }])
                }
            }
        }
        for (id, state) in old_state.iter() {
            for (pid, access) in state.projects.iter() {
                actions.extend([Actions {
                    subject_entity_id: *id,
                    object_entity_id: *pid,
                    object_entity_type: ObjectEntityType::Project,
                    access: *access,
                    action: Action::Delete,
                    subject_entity_type: state.entity.clone(),
                }])
            }
            for (gid, access) in state.namespaces.iter() {
                actions.extend([Actions {
                    subject_entity_id: *id,
                    object_entity_id: *gid,
                    object_entity_type: ObjectEntityType::Group,
                    access: *access,
                    action: Action::Delete,
                    subject_entity_type: state.entity.clone(),
                }])
            }
        }

        actions
    }

    fn compare_ownerships(
        entity_type: EntityType,
        mut old_state: HashMap<u64, AccessLevel>,
        new_state: HashMap<u64, AccessLevel>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        for (id, access) in new_state.iter() {
            if old_state.contains_key(id) {
                if old_state[id] != new_state[id] {
                    actions.extend([Actions {
                        subject_entity_id: user_id,
                        object_entity_id: *id,
                        object_entity_type: ObjectEntityType::Group,
                        access: *access,
                        action: Action::Update,
                        subject_entity_type: entity_type.clone(),
                    }]);
                }
                old_state.remove(id);
            } else {
                actions.extend([Actions {
                    subject_entity_id: user_id,
                    object_entity_id: *id,
                    object_entity_type: ObjectEntityType::Group,
                    access: *access,
                    action: Action::Create,
                    subject_entity_type: entity_type.clone(),
                }])
            }
        }
        for (id, access) in old_state.iter() {
            actions.extend([Actions {
                subject_entity_id: user_id,
                object_entity_id: *id,
                object_entity_type: ObjectEntityType::Group,
                access: *access,
                action: Action::Delete,
                subject_entity_type: entity_type.clone(),
            }])
        }
    }

    fn compare_projects(
        entity_type: EntityType,
        mut old_state: HashMap<u64, AccessLevel>,
        new_state: HashMap<u64, AccessLevel>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        for (id, access) in new_state.iter() {
            if old_state.contains_key(id) {
                if old_state[id] != new_state[id] {
                    actions.extend([Actions {
                        subject_entity_id: user_id,
                        object_entity_id: *id,
                        object_entity_type: ObjectEntityType::Project,
                        access: *access,
                        action: Action::Update,
                        subject_entity_type: entity_type.clone(),
                    }]);
                }
                old_state.remove(id);
            } else {
                actions.extend([Actions {
                    subject_entity_id: user_id,
                    object_entity_id: *id,
                    object_entity_type: ObjectEntityType::Project,
                    access: *access,
                    action: Action::Create,
                    subject_entity_type: entity_type.clone(),
                }])
            }
        }
        for (id, access) in old_state.iter() {
            actions.extend([Actions {
                subject_entity_id: user_id,
                object_entity_id: *id,
                object_entity_type: ObjectEntityType::Project,
                access: *access,
                action: Action::Delete,
                subject_entity_type: entity_type.clone(),
            }])
        }
    }
}
