use crate::{
    gitlab::{
        apis::{
            groups::GitlabGroupsApi, members::GitlabMembersApi, projects::GitlabProjectsApi,
            users::GitlabUsersApi,
        },
        GitlabApiInterface,
    },
    output::{out_message::OutMessage, out_spinner::OutSpinner},
    types::v1::{
        access_level::AccessLevel,
        config::Config,
        config_file::ConfigFile,
        group::Group,
        namespace::Namespace,
        project::Project,
        state::{AccessUnit, EntityType, State},
        user::User,
    },
};
use gitlab::Gitlab;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

// SyncService should be used to sync config with GitLab
pub(crate) struct SyncService<T: GitlabApiInterface> {
    config_file: ConfigFile,
    gitlab_api: T,
    state_source: String,
    state_destination: String,
    write_state: bool,
    file_name: String,
    state: State,
}

#[derive(Debug, Clone)]
enum ObjectEntityType {
    Project,
    Group,
}

#[derive(Debug, Clone)]
pub(crate) struct Actions {
    subject_entity_id: u64,
    subject_entity_type: EntityType,
    object_entity_id: u64,
    object_entity_type: ObjectEntityType,
    access: AccessLevel,
    action: Action,
}
#[derive(Debug, Clone)]
enum Action {
    Create,
    Delete,
    Update,
}

impl<T: GitlabApiInterface> SyncService<T> {
    pub(crate) fn new(gitlab_api: T) -> Self {
        Self {
            config_file: Default::default(),
            gitlab_api,
            state_source: todo!(),
            state_destination: todo!(),
            write_state: todo!(),
            file_name: todo!(),
            state: State::default(),
        }
    }

    pub(crate) fn compare(&self) -> Result<()> {
        let mut old_state: HashMap<u64, AccessUnit> = HashMap::new();
        if !self.state_source.is_empty() {
            OutMessage::message_info_with_alias(
                format!("I will try to use this file: {}", self.state_source.clone()).as_str(),
            );
            old_state = AccessUnit::read_from_file(self.state_source.clone())?
        } else {
            if self.config_file.state.as_str() == "~" || self.config_file.state.is_empty() {
                OutMessage::message_info_with_alias(
                    "State is not found, I will generate a new one",
                );
            } else {
                OutMessage::message_info_with_alias("State is found");
                old_state = match serde_json::from_str(self.config_file.state.as_str()) {
                    Ok(state) => state,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidData, err)),
                };
            }
        }

        let mut new_state: HashMap<u64, AccessUnit> = HashMap::new();
        for u in self.config_file.config.users.iter().clone() {
            new_state.insert(
                u.id,
                AccessUnit {
                    projects: self.configure_projects(u, self.config_file.config.clone()),
                    namespaces: self.configure_groups(u, self.config_file.config.clone()),
                    entity: EntityType::User,
                },
            );
        }
        for u in self.config_file.config.groups.iter().clone() {
            new_state.insert(
                u.id,
                AccessUnit {
                    projects: self.gr_configure_projects(u),
                    namespaces: self.gr_configure_groups(u),
                    entity: EntityType::Group,
                },
            );
        }

        Ok(())
    }

    pub(crate) fn apply(
        &self,
        actions: Vec<Actions>,
        gitlab_client: &Gitlab,
        state: &mut HashMap<u64, AccessUnit>,
        dry: bool,
    ) -> Result<()> {
        for a in actions.iter() {
            let users_api = self.gitlab_api.users();
            let projects_api = self.gitlab_api.projects();
            let groups_api = self.gitlab_api.groups();
            let members_api = self.gitlab_api.members();

            let subject_name = match a.subject_entity_type {
                EntityType::User => match users_api.get_data_by_id(a.subject_entity_id) {
                    Ok(r) => r.name,
                    Err(err) => return Err(err),
                },
                EntityType::Group => match users_api.get_data_by_id(a.subject_entity_id) {
                    Ok(r) => r.name,
                    Err(err) => return Err(err),
                },
            };

            match a.object_entity_type {
                ObjectEntityType::Project => {
                    let project = match projects_api.get_data_by_id(a.object_entity_id) {
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
                                        match members_api.add_user_to_project(
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
                                        match members_api.add_group_to_project(
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
                                        match members_api.remove_user_from_project(
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
                                        match members_api.remove_group_from_project(
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
                                        match members_api.edit_user_in_project(
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
                                        match members_api.remove_group_from_project(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(_) => {
                                                match members_api.add_group_to_project(
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
                    let group = match groups_api.get_data_by_id(a.object_entity_id) {
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
                                        match members_api.add_user_to_group(
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
                                        match members_api.add_group_to_namespace(
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
                                        match members_api.remove_user_from_group(
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
                                        match members_api.remove_group_from_namespace(
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
                                        match members_api.edit_user_in_group(
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
                                        match members_api.remove_group_from_namespace(
                                            a.subject_entity_id,
                                            a.object_entity_id,
                                        ) {
                                            Err(err) => {
                                                spinner.spinner_failure(err.to_string());
                                                return Err(err);
                                            }
                                            Ok(_) => {
                                                match members_api.add_group_to_namespace(
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

    pub(crate) fn write_state(&self) -> Result<()> {
        if self.write_state {
            match AccessUnit::write_to_file(
                self.state.data().clone(),
                self.state_destination.clone(),
            ) {
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

        match self.config_file.write(self.file_name.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn configure_projects(&self, u: &User, c: Config) -> HashMap<u64, AccessLevel> {
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
                    self.higher_access(p.access_level, *keys.get(&p.id).unwrap()),
                );
            }
        }
        projects.clear();
        for (k, v) in keys.iter() {
            projects_map.insert(*k, *v);
        }
        return projects_map;
    }

    pub(crate) fn configure_groups(&self, u: &User, c: Config) -> HashMap<u64, AccessLevel> {
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
                    self.higher_access(g.access_level, *keys.get(&g.id).unwrap()),
                );
            }
        }
        groups.clear();
        for (k, v) in keys.iter() {
            groups_map.insert(*k, *v);
        }
        groups_map
    }

    fn gr_configure_groups(&self, u: &Group) -> HashMap<u64, AccessLevel> {
        let mut groups_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut groups: Vec<Namespace> = u.namespaces.clone();
        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for g in groups.iter() {
            if !keys.contains_key(&g.id) {
                keys.insert(g.id, g.clone().access_level);
            } else {
                keys.insert(
                    g.id,
                    self.higher_access(g.access_level, *keys.get(&g.id).unwrap()),
                );
            }
        }
        groups.clear();
        for (k, v) in keys.iter() {
            groups_map.insert(*k, *v);
        }
        groups_map
    }

    fn gr_configure_projects(&self, u: &Group) -> HashMap<u64, AccessLevel> {
        let mut projects_map: HashMap<u64, AccessLevel> = HashMap::new();
        let mut projects: Vec<Project> = u.projects.clone();
        let mut keys: HashMap<u64, AccessLevel> = HashMap::new();
        for p in projects.iter() {
            if !keys.contains_key(&p.id) {
                keys.insert(p.id, p.clone().access_level);
            } else {
                keys.insert(
                    p.id,
                    self.higher_access(p.access_level, *keys.get(&p.id).unwrap()),
                );
            }
        }
        projects.clear();
        for (k, v) in keys.iter() {
            projects_map.insert(*k, *v);
        }
        return projects_map;
    }

    fn higher_access(&self, a1: AccessLevel, a2: AccessLevel) -> AccessLevel {
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

    pub(crate) fn compare_states(
        &self,
        mut old_state: HashMap<u64, AccessUnit>,
        new_state: HashMap<u64, AccessUnit>,
    ) -> Vec<Actions> {
        let mut actions: Vec<Actions> = Vec::new();
        for (id, state) in new_state.iter() {
            if old_state.contains_key(id) {
                self.compare_projects(
                    state.entity.clone(),
                    old_state[id].projects.clone(),
                    state.projects.clone(),
                    &mut actions,
                    *id,
                );
                self.compare_ownerships(
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
        &self,
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
        &self,
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
