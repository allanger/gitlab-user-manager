use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitlabGroup {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) web_url: String,
}

impl GitlabGroup {
    fn get_data_by_id() {}
    fn add_user() {}
    fn update_user() {}
    fn remove_user() {}
}
