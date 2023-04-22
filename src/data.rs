use lenra_app::api::Doc;
use serde::{Deserialize, Serialize};

/** Counter data */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Counter {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub count: u32,
    pub user: String,
}

impl Doc for Counter {
    fn id(&self) -> Option<String> {
        match &self.id {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}
