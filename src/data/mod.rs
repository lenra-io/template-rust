use serde::{Deserialize, Serialize};

use self::service::Data;

pub mod service;

/** Counter data */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Counter {
    #[serde(rename = "_id")]
    pub id: Option<u32>,
    #[serde(rename = "_datastore")]
    pub datastore: Option<String>,
    pub count: Option<u32>,
}

impl Data for Counter {
    fn id(&self) -> Option<u32> {
        self.id
    }

    fn datastore(&self) -> Option<String> {
        let ref this = self.datastore;
        match this {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }
}
