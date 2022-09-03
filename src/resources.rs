use std::{
    collections::HashMap,
    io::{self, Write},
};

use serde::{Deserialize, Serialize};

macro_rules! add_resource {
    ($path: tt) => {
        ($path, include_bytes!($path))
    };
}

pub const RESOURCE_MAP: [(&str, &[u8]); 1] = [add_resource!("logo-vertical.png")];

/** Lenra widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource: String,
}

impl Resource {
    pub fn handle(&self) {
        let map = HashMap::from(RESOURCE_MAP);
        io::stdout()
            .write_all(map.get(self.resource.as_str()).unwrap())
            .unwrap();
    }
}
