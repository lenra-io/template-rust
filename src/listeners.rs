// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::{service::Api, Counter};

const COUNTER_DATASTORE: &str = "counter";
const DATASTORES: [&str; 1] = [COUNTER_DATASTORE];

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct UnknownListener {
    pub action: String,
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: Option<Value>,
}

/** Lenra widget request */
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum Listener {
    OnEnvStart(BaseListener),
    Increment(Increment),
}

impl Listener {
    pub fn handle(&self) {
        match self {
            Listener::Increment(inc) => inc.handle(), /* .await */
            Listener::OnEnvStart(listener) => create_datastores(&listener.api),
        }
    }
}

/** Lenra listener request */
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct BaseListener {
    pub props: Option<Value>,
    pub event: Option<Value>,
    pub api: Api,
}

// #[async_trait]
trait ListenerHandler {
    /* async */
    fn handle(&self);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Increment {
    pub props: IncrementProps,
    pub event: Option<Value>,
    pub api: Api,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IncrementProps {
    id: u32,
    datastore: String,
}

// #[async_trait]
impl ListenerHandler for Increment {
    /* async */
    fn handle(&self) {
        let mut counter: Counter = self
            .api
            .get_data(self.props.datastore.clone(), self.props.id)
            .unwrap();
        counter.count = counter.count + 1;
        self.api.update_data(counter).unwrap();
    }
}

fn create_datastores(api: &Api) {
    DATASTORES.iter().for_each(|&datastore| {
        api.create_datastore(datastore)
            .expect(format!("Failed creating datastore {}", datastore).as_str())
    });
    api.create_data(Counter {
        _id: None,
        _datastore: COUNTER_DATASTORE.into(),
        count: 0,
    }).unwrap();
}
