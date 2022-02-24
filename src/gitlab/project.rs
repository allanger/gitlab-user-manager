use gitlab::{Gitlab, api::{projects, ApiError, Query}};
use serde::Deserialize;
use core::time;
use std::{io::{Result, Error, ErrorKind}, thread};

#[derive(Debug, Deserialize)]
struct Project {
    pub(crate) id: u64,
    pub(crate) name: String,
}

impl Project {
    pub(crate) fn get_data_by_id(&self, gitlab_client: &Gitlab, id: u64) -> Result<()> {
        let project = match projects::Project::builder().project(id).build() {
            Ok(project) => project,
            Err(_error) => {
                return Err(Error::new(std::io::ErrorKind::Other, _error.to_string()));
            }
        };

        let output: Project = match project.query(gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                println!("Gitlab is screw by amount of our requests. I'm sorry, buy you need to wait, mate");
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                return self.get_data_by_id(gitlab_client, id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, "asd"));
            }
            Ok(res) => res,
        };
        // self = output;
        Ok(())
    }
    fn add_user() {}
    fn update_user() {}
    fn remove_user() {}
}
