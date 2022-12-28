use std::{
    collections::HashMap,
    io::{self, Write},
};

use serde::{Deserialize, Serialize};

macro_rules! resource {
    ($path: tt) => {
        ($path, include_bytes!($path))
    };
}

pub const RESOURCE_MAP: [(&str, &[u8]); 1] = [resource!("logo.png")];

/** Lenra view request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Resource {
    pub resource: String,
}

impl Resource {
    pub fn handle(&self) {
        let map: HashMap<&str, &[u8]> = HashMap::from(RESOURCE_MAP);
        io::stdout()
            .write_all(map.get(self.resource.as_str()).unwrap())
            .unwrap();
    }
}
