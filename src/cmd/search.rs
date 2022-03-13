pub(crate) mod commands;
mod groups;
mod projects;
mod users;

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    io::{Error, Result},
    str::FromStr,
};

use gitlab::Gitlab;

use self::{groups::Groups, projects::Projects, users::Users};

pub(crate) trait SearchEntity {
    fn search(&self, query: &str) -> Result<()>;
}

pub(crate) struct SearchService<'a> {
    entities: HashMap<EntityName, Box<dyn SearchEntity + 'a>>,
}

impl<'a> SearchService<'a> {
    pub fn new(gitlab_client: &'a Gitlab) -> Self {
        let mut entities: HashMap<EntityName, Box<dyn SearchEntity>> = HashMap::new();
        entities.insert(EntityName::PROJECTS, Box::new(Projects::new(gitlab_client)));
        entities.insert(EntityName::GROUPS, Box::new(Groups::new(gitlab_client)));
        entities.insert(EntityName::USERS, Box::new(Users::new(gitlab_client)));
        SearchService { entities }
    }

    pub fn search(&self, entity_name: &str, query: &str) -> Result<()> {
        let entity_name = EntityName::from_str(entity_name)?;
        let entity = self.entities.get(&entity_name).ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                format!("Could not resolve entity with name {entity_name}"),
            )
        })?;

        entity.search(query)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub(crate) enum EntityName {
    GROUPS,
    PROJECTS,
    USERS,
}

impl FromStr for EntityName {
    type Err = Error;

    fn from_str(s: &str) -> Result<EntityName> {
        match s {
            "groups" => Ok(Self::GROUPS),
            "projects" => Ok(Self::PROJECTS),
            "users" => Ok(Self::USERS),
            name => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Entity with name {name} does not exist"),
            )),
        }
    }
}

impl Display for EntityName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
