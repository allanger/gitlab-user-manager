use std::io::Error;

use gitlab::Gitlab;

struct RsGitlab {
    gitlab_client: Gitlab,
}

pub struct GitlabConnection {
    pub url: String,
    pub token: String,
}

pub fn new_gitlab_client(g: GitlabConnection) -> impl GitlabActions {
    RsGitlab {
        gitlab_client: Gitlab::new(g.url, g.token).unwrap(),
    }
}

pub trait GitlabActions {
    fn get_project_name_by_id(&self) -> Result<(), Error>;
}

impl GitlabActions for RsGitlab {
    fn get_project_name_by_id(&self) -> Result<(), Error> {
        println!("GETTING PROJECT ID");
        Ok(())
    }
}
