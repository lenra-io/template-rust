use lenra_app::components::lenra::*;
use lenra_app::components::listener;
use lenra_app::view::{ViewParams, ViewResponse};
use lenra_app::{api::Doc, view::ViewResponseGenerator};
use lenra_app::{props, Result};
use serde::{Deserialize, Serialize};

use crate::data::Counter;
use crate::listeners::IncrementProps;

pub fn counter(params: ViewParams<Vec<Counter>, CounterViewProps>) -> Result<ViewResponse> {
    let counters = params.data.unwrap();
    let counter = counters.get(0).unwrap();
    let props = params.props.unwrap();

    let result: LenraComponent = flex(vec![
        text(format!("{}: {}", props.text, counter.count)).try_into()?,
        button("+")
            .on_pressed(Some(
                listener("increment")
                    .props(Some(
                        IncrementProps {
                            id: counter.id().unwrap().clone(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .into(),
    ])
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterViewProps {
    pub text: String,
}

props!(CounterViewProps);
