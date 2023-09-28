use lenra_app::api::Doc;
use lenra_app::components::listener;
use lenra_app::view::{ViewParams, ViewResponse, ViewResponseGenerator};
use lenra_app::{ComponentBuilder, Result};
use serde_json::json;
use serde_json::Value;

use crate::data::Counter;
use crate::listeners::IncrementProps;

pub fn counter(params: ViewParams<Vec<Counter>, Value>) -> Result<ViewResponse> {
    let counters = params.data.unwrap();
    let counter = counters.get(0).unwrap();

    Ok(json!({
        "value": counter.count,
        "onIncrement": listener("increment")
            .props(Some(
                IncrementProps {
                    id: counter.id().unwrap().clone(),
                }
                .into(),
            ))
            .build(),
    })
    .gen())
}
