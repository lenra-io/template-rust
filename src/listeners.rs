use lenra_app::{
    api::Api,
    listener::{Listener, ListenerParams, SystemEvents},
    Handler, Result,
};
// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::Counter;

pub const COUNTER_COLLECTION: &str = "counter";
pub const GLOBAL_USER: &str = "global";
pub const CURRENT_USER: &str = "@me";

pub fn get_listeners() -> Vec<Listener> {
    vec![
        Listener::new(
            SystemEvents::OnEnvStart.to_str(),
            |params: ListenerParams| create_counter(&params.api, GLOBAL_USER),
        ),
        Listener::new(
            SystemEvents::OnUserFirstJoin.to_str(),
            |params: ListenerParams| create_counter(&params.api, CURRENT_USER),
        ),
        Listener::new("increment", increment),
    ]
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IncrementProps {
    id: String,
}

fn increment(params: ListenerParams<IncrementProps, Value>) -> Result<()> {
    let mut counter: Counter = params
        .api
        .data
        .get_doc(COUNTER_COLLECTION, params.props.unwrap().id.as_str())?;
    counter.count = counter.count + 1;
    params.api.data.update_doc(COUNTER_COLLECTION, counter)?;
    Ok(())
}

fn create_counter(api: &Api, user: &str) -> Result<()> {
    api.data.create_doc(
        COUNTER_COLLECTION,
        Counter {
            count: 0,
            user: user.into(),
            ..Default::default()
        },
    )?;
    Ok(())
}
