use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::{service::Api, Counter};

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
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum Listener {
    Increment(Increment),
}

impl Listener {
    pub async fn handle(&self) {
        match self {
            Listener::Increment(inc) => inc.handle().await,
        }
    }
}

#[async_trait]
trait ListenerHandler {
    async fn handle(&self);
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

#[async_trait]
impl ListenerHandler for Increment {
    async fn handle(&self) {
        let mut counter: Counter = self
            .api
            .get_data(self.props.datastore.clone(), self.props.id)
            .await
            .unwrap();
        counter.count = counter.count + 1;
    }
}
