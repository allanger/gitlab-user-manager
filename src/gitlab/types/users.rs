use serde::Deserialize;
use tabled::Tabled;

#[derive(Debug, Deserialize, Tabled)]
pub(crate) struct User {
    pub(crate) id: u64,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) web_url: String,
}
