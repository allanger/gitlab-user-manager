use serde::Deserialize;
use tabled::Tabled;

#[derive(Debug, Deserialize, Tabled, Clone)]
pub(crate) struct CustomMember {
    pub(crate) id: u64,
    pub(crate) access_level: gitlab::AccessLevel,
    pub(crate) username: String,
    pub(crate) name: String,
    pub(crate) web_url: String,
}
