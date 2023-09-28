use lenra_app::{
    api::{Api, CollectionGetter},
    components::lenra::{DataProjection, DataQuery},
    listener::{Listener, ListenerParams, SystemEvents},
    props, Handler, Result,
};
// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
        Listener::new(
            SystemEvents::OnSessionStart.to_str(),
            |_params: ListenerParams| Ok(()),
        ),
        Listener::new("increment", increment),
    ]
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IncrementProps {
    pub id: String,
}
props!(IncrementProps);

fn increment(params: ListenerParams<IncrementProps, Value>) -> Result<()> {
    params
        .api
        .data
        .coll(COUNTER_COLLECTION)
        .update_many::<Counter, DataQuery, Value>(
            serde_json::from_value(json!({ "_id": params.props.unwrap().id }))?,
            json!({
                "$inc": {
                    "count": 1
                }
            }),
        )?;
    Ok(())
}

fn create_counter(api: &Api, user: &str) -> Result<()> {
    let coll = api.data.coll(COUNTER_COLLECTION);
    let query: DataQuery = serde_json::from_value(json!({ "user": user }))?;
    let counters: Vec<Counter> = coll.find(query, None::<DataProjection>)?;
    if counters.is_empty() {
        api.data.coll(COUNTER_COLLECTION).create_doc(Counter {
            count: 0,
            user: user.into(),
            ..Default::default()
        })?;
    }
    Ok(())
}
