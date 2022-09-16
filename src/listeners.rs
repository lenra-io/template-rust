// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::data::{service::Api, Counter};

pub const COUNTER_COLLECTION: &str = "counter";
pub const GLOBAL_USER: &str = "global";
pub const CURRENT_USER: &str = "@me";

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
    OnUserFirstJoin(BaseListener),
    Increment(Increment),
}

impl Listener {
    pub fn handle(&self) {
        log::debug!("Listener: {:?}", self);
        match self {
            Listener::Increment(inc) => inc.handle(),
            Listener::OnEnvStart(listener) => create_counter(&listener.api, GLOBAL_USER),
            Listener::OnUserFirstJoin(listener) => create_counter(&listener.api, CURRENT_USER),
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
    id: String,
}

// #[async_trait]
impl ListenerHandler for Increment {
    /* async */
    fn handle(&self) {
        let mut counter: Counter = self
            .api
            .get_doc(COUNTER_COLLECTION, self.props.id.as_str())
            .unwrap();
        counter.count = counter.count + 1;
        self.api.update_doc(COUNTER_COLLECTION, counter).unwrap();
    }
}

fn create_counter(api: &Api, user: &str) {
    let res = api.create_doc(
        COUNTER_COLLECTION,
        json!({
            "count": 0,
            "user": user,
        }),
    );
    if let Err(error) = res {
        log::warn!("Error occured while creating the counter: {}", error);
    }
}
