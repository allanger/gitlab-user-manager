use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitlabUser {
    pub(crate) id: u64,
    pub(crate) name: String,
}

impl GitlabUser {
    fn get_data_by_id() {}
}
