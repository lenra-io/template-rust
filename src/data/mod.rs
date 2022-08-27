use serde::{Deserialize, Serialize};

use self::service::Data;

pub mod service;

/** Counter data */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Counter {
    pub _id: Option<u32>,
    pub _datastore: String,
    pub count: u32,
}

impl Data for Counter {
    fn id(&self) -> Option<u32> {
        self._id
    }

    fn datastore(&self) -> String {
        self._datastore.clone()
    }
}
