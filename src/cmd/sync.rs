use std::io::{Error, ErrorKind};

use clap::{App, Arg, ArgMatches};
use gitlab::Gitlab;

use crate::files::state_exists;

use crate::{cmd::Cmd, files, types::state};

use self::sync_cmd::{apply, compare_states, configure_projects};

use super::args::{arg_gitlab_token, arg_gitlab_url};

/// init cmd should be used to generate an empty gum-config
pub(crate) fn add_sync_cmd() -> App<'static> {
    let dry_run = Arg::new("dry-run")
        .short('d')
        .takes_value(false)
        .help("Use if you wanna see what's gonna happen without applying new configuration");
    return App::new("sync")
        .about("Sync your config file with GitLab and generate the state file")
        .arg(dry_run)
        .arg(arg_gitlab_token())
        .arg(arg_gitlab_url());
}

pub(crate) struct SyncCmd {
    dry_run: bool,
    gitlab_client: Gitlab,
}

pub(crate) fn prepare<'a>(sub_matches: &'a ArgMatches) -> Result<impl Cmd<'a>, Error> {
    let dry_run: bool = sub_matches.is_present("dry-run");
    let gitlab_client: Gitlab;
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
        Ok(g) => g,
        Err(_err) => return Err(Error::new(ErrorKind::Other, _err)),
    };
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
        // Generate new state
        let mut new_state: Vec<state::State> = Vec::new();
        let mut old_state: Vec<state::State>;

        for u in config.users.iter().clone() {
            let user_state = state::State {
                projects: configure_projects(u, config.clone()),
                ownerships: u.ownerships.clone(),
                user_id: u.id,
            };
            new_state.extend([user_state]);
        }
        // Read the old state
        if state_exists() {
            old_state = match files::read_state() {
                Ok(s) => s,
                Err(_error) => return Err(_error),
            };
        } else {
            old_state = Vec::new();
        }

        let actions = compare_states(old_state.clone(), new_state);

        match apply(actions, &self.gitlab_client, &mut old_state, self.dry_run) {
            Ok(_) => match files::write_state(old_state, self.dry_run) {
                Ok(_) => Ok(()),
                Err(_err) => return Err(_err),
            },
            Err(_err) => return Err(_err),
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

    use gitlab::Gitlab;

    use crate::gitlab::GitlabActions;
    use crate::types::access_level::AccessLevel;

    use crate::types::{
        config::Config,
        ownership::Ownership,
        project::Project,
        state::State,
        user::{self},
    };

    pub(crate) fn apply(
        actions: Vec<Actions>,
        gitlab_client: &Gitlab,
        state: &mut Vec<State>,
        dry: bool,
    ) -> Result<(), Error> {
        for a in actions.iter() {
            let gitlab = crate::gitlab::new_gitlab_client(gitlab_client.to_owned());
            let username = match gitlab.get_user_data_by_id(a.user_id) {
                Ok(r) => r,
                Err(_error) => return Err(_error),
            };
            match a.entity_type {
                EntityType::PROJECT => {
                    let project = match gitlab.get_project_data_by_id(a.entity_id) {
                        Ok(r) => r,
                        Err(_error) => return Err(_error),
                    };
                    match a.action {
                        Action::CREATE => {
                            println!(
                                "Adding {} to {} as {}",
                                username.name, project.name, a.access
                            );
                            if !dry {
                                todo!();
                            }
                            let mut exist = false;
                            for (i, _) in state.clone().iter().enumerate() {
                                if state[i].user_id == a.user_id {
                                    exist = true;
                                    state[i].projects.extend([Project {
                                        name: project.name.to_string(),
                                        id: project.id,
                                        access_level: a.access,
                                    }]);
                                };
                            }
                            if !exist {
                                state.extend([State {
                                    user_id: a.user_id,
                                    ownerships: vec![],
                                    projects: vec![Project {
                                        name: project.name.to_string(),
                                        id: project.id,
                                        access_level: a.access,
                                    }],
                                }])
                            }
                        }
                        Action::DELETE => {
                            println!(
                                "Deleting {} from {} as {}",
                                username.name, project.name, a.access
                            );
                            if !dry {
                                todo!();
                            }
                            for (i, s) in state.clone().iter().enumerate() {
                                if s.user_id == a.user_id {
                                    let mut ui = 0;
                                    for p in s.projects.iter() {
                                        if p.id != a.entity_id {
                                            state[i].projects[ui] = p.clone();
                                            ui += 1;
                                        }
                                    }
                                    state[i].projects.drain(ui..);
                                }
                            }
                        }
                        Action::UPDATE => {
                            println!(
                                "Updating {} in {} to {}",
                                username.name, project.name, a.access
                            );
                            if !dry {
                                todo!();
                            }
                            for (i, s) in state.clone().iter().enumerate() {
                                if s.user_id == a.user_id {
                                    for (pi, p) in s.projects.iter().enumerate() {
                                        if p.id != a.entity_id {
                                            state[i].projects[pi].access_level = a.access;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                EntityType::GROUP => {
                    let group = match gitlab.get_group_data_by_id(a.entity_id) {
                        Ok(r) => r,
                        Err(_error) => return Err(_error),
                    };
                    match a.action {
                        Action::CREATE => {
                            println!("Adding {} to {} as {}", username.name, group.name, a.access);
                            if !dry {
                                todo!();
                            }
                            let mut exist = false;
                            for (i, _) in state.clone().iter().enumerate() {
                                if state[i].user_id == a.user_id {
                                    exist = true;
                                    state[i].ownerships.extend([Ownership {
                                        name: group.name.to_string(),
                                        id: group.id,
                                        url: group.web_url.to_string(),
                                    }]);
                                };
                            }
                            if !exist {
                                state.extend([State {
                                    user_id: a.user_id,
                                    projects: vec![],
                                    ownerships: vec![Ownership {
                                        name: group.name.to_string(),
                                        id: group.id,
                                        url: group.web_url.to_string(),
                                    }],
                                }])
                            }
                        }
                        Action::DELETE => {
                            println!(
                                "Deleting {} from {} as {}",
                                username.name, group.name, a.access
                            );
                            if !dry {
                                todo!();
                            }
                            for (i, s) in state.clone().iter().enumerate() {
                                if s.user_id == a.user_id {
                                    let mut ui = 0;
                                    for o in s.ownerships.iter() {
                                        if o.id != a.entity_id {
                                            state[i].ownerships[ui] = o.clone();
                                            ui += 1;
                                        }
                                    }
                                    state[i].ownerships.drain(ui..);
                                }
                            }
                        }
                        Action::UPDATE => {
                            println!("Groups can't be updated yet, because only owner access is allowed for groups");
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub(crate) fn configure_projects<'a>(u: &user::User, c: Config) -> Vec<Project> {
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

        if project1.access_level == AccessLevel::MAINTAINER
            || project2.access_level == AccessLevel::MAINTAINER
        {
            access_level = AccessLevel::MAINTAINER;
        } else if project1.access_level == AccessLevel::DEVELOPER
            || project2.access_level == AccessLevel::DEVELOPER
        {
            access_level = AccessLevel::DEVELOPER;
        } else if project1.access_level == AccessLevel::REPORTER
            || project2.access_level == AccessLevel::REPORTER
        {
            access_level = AccessLevel::REPORTER;
        } else {
            access_level = AccessLevel::GUEST;
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
        mut old_state: Vec<State>,
        mut new_state: Vec<State>,
    ) -> Vec<Actions> {
        let mut actions: Vec<Actions> = Vec::new();
        let mut tni = 0;
        for n in new_state.clone().iter() {
            let mut toi = 0;
            let mut found = false;
            for o in old_state.clone().iter() {
                if o.user_id == n.user_id {
                    found = true;
                    compare_projects(
                        o.projects.clone(),
                        n.projects.clone(),
                        &mut actions,
                        n.user_id,
                    );
                    compare_ownerships(
                        o.ownerships.clone(),
                        n.ownerships.clone(),
                        &mut actions,
                        n.user_id,
                    );
                } else {
                    old_state[toi] = o.clone();
                    toi += 1;
                }
                // If user is not found in the new state -> add remove action
            }
            old_state = old_state[..toi].to_vec();
            if !found {
                new_state[tni] = n.clone();
                tni += 1;
            }
        }
        new_state = new_state[..tni].to_vec();

        new_state.iter().for_each(|p| {
            for np in p.projects.iter() {
                actions.extend([Actions {
                    user_id: p.user_id,
                    entity_id: np.id,
                    entity_type: EntityType::PROJECT,
                    access: np.access_level,
                    action: Action::CREATE,
                }])
            }
            for ng in p.ownerships.iter() {
                actions.extend([Actions {
                    user_id: p.user_id,
                    entity_id: ng.id,
                    entity_type: EntityType::GROUP,
                    access: AccessLevel::OWNER,
                    action: Action::CREATE,
                }])
            }
        });

        old_state.iter().for_each(|p| {
            for op in p.projects.iter() {
                actions.extend([Actions {
                    user_id: p.user_id,
                    entity_id: op.id,
                    entity_type: EntityType::PROJECT,
                    access: op.access_level,
                    action: Action::DELETE,
                }])
            }
            for og in p.ownerships.iter() {
                actions.extend([Actions {
                    user_id: p.user_id,
                    entity_id: og.id,
                    entity_type: EntityType::GROUP,
                    access: AccessLevel::OWNER,
                    action: Action::DELETE,
                }])
            }
        });

        // actions.iter().for_each(|f| println!("{:?}", f));
        actions
    }

    fn compare_ownerships(
        mut old_state: Vec<Ownership>,
        mut new_state: Vec<Ownership>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        let mut tni = 0;
        for nv in new_state.clone().iter() {
            let mut found = false;
            let mut toi = 0;
            for ov in old_state.clone().iter_mut() {
                if nv.id == ov.id {
                    found = true;
                } else {
                    old_state[toi] = ov.clone();
                    toi += 1;
                }
            }
            old_state = old_state[..toi].to_vec();
            if !found {
                new_state[tni] = nv.clone();
                tni += 1;
            }
        }
        new_state = new_state[..tni].to_vec();
        for nv in new_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: nv.id,
                entity_type: EntityType::GROUP,
                access: AccessLevel::OWNER,
                action: Action::CREATE,
            }])
        }
        for ov in old_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: ov.id,
                entity_type: EntityType::GROUP,
                access: AccessLevel::OWNER,
                action: Action::DELETE,
            }])
        }
    }

    fn compare_projects(
        mut old_state: Vec<Project>,
        mut new_state: Vec<Project>,
        actions: &mut Vec<Actions>,
        user_id: u64,
    ) {
        // Temporary new index
        let mut tni = 0;
        for nv in new_state.clone().iter() {
            let mut found = false;
            let mut toi = 0;
            for ov in old_state.clone().iter_mut() {
                if nv.id == ov.id {
                    found = true;
                    if nv.access_level != ov.access_level {
                        actions.extend([Actions {
                            user_id,
                            entity_id: nv.id,
                            entity_type: EntityType::PROJECT,
                            access: nv.access_level,
                            action: Action::UPDATE,
                        }]);
                    }
                    // break;
                } else {
                    old_state[toi] = ov.clone();
                    toi += 1;
                }
            }
            old_state = old_state[..toi].to_vec();
            if !found {
                new_state[tni] = nv.clone();
                tni += 1;
            }
        }
        new_state = new_state[..tni].to_vec();
        for nv in new_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: nv.id,
                entity_type: EntityType::PROJECT,
                access: nv.access_level,
                action: Action::CREATE,
            }])
        }
        for ov in old_state.iter() {
            actions.extend([Actions {
                user_id,
                entity_id: ov.id,
                entity_type: EntityType::PROJECT,
                access: ov.access_level,
                action: Action::DELETE,
            }])
        }
    }
}
