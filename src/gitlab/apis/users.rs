use gitlab::{Gitlab, api::{users, ApiError, Query}};
use core::time;
use std::{io::{Result, Error, ErrorKind}, thread};

use crate::{gitlab::types::users::User, output::{out_message::OutMessage, out_spinner::OutSpinner}};
pub(crate) trait GitlabUsersApi {
    fn get_data_by_id(&self, id: u64) -> Result<User>;
}

pub(crate) struct UserGitlab {
    pub(crate) gitlab_client: Gitlab,
}

impl UserGitlab {
    pub(crate) fn new(gitlab_client: Gitlab) -> Self {
        Self { gitlab_client }
    }
}

impl GitlabUsersApi for UserGitlab {
    fn get_data_by_id(&self, id: u64) -> Result<User> {
        let user = match users::User::builder().user(id).build() {
            Ok(user) => user,
            Err(err) => {
                return Err(Error::new(std::io::ErrorKind::Other, err.to_string()));
            }
        };
        let output: User = match user.query(&self.gitlab_client) {
            Err(err) => {
                match err {
                    ApiError::GitlabObject { ref obj } => {
                        if format!("{}", obj) == "{\"error\":\"This endpoint has been requested too many times. Try again later.\"}" {
                                OutMessage::message_info_with_alias("Gitlab is screwed by amount of your requests. You need to wait");
                                let spinner = OutSpinner::spinner_start("Waiting 30s".to_string());
                                let await_time = time::Duration::from_secs(30);
                                thread::sleep(await_time);
                                spinner.spinner_success("Let's try again".to_string());
                                return self.get_data_by_id(id);
                            };
                    }
                    _ => return Err(Error::new(ErrorKind::AddrNotAvailable, err)),
                };
                return Err(Error::new(ErrorKind::AddrNotAvailable, err));
            }
            Ok(res) => res,
        };

        Ok(output)
    }
}
