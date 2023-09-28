use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use serde_json::json;

use crate::{
    listeners::{COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER},
    views::lenra::counter::CounterViewProps,
};

pub fn home(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![
        view("lenra:counter")
            .find(Some(
                ViewDefinitionsFind::builder()
                    .coll(COUNTER_COLLECTION)
                    .query(json!({ "user": CURRENT_USER }))
                    .try_into()?,
            ))
            .props(Some(
                CounterViewProps {
                    text: "My personnal counter".into(),
                }
                .try_into()?,
            ))
            .try_into()?,
        view("lenra:counter")
            .find(Some(
                ViewDefinitionsFind::builder()
                    .coll(COUNTER_COLLECTION)
                    .query(json!({ "user": GLOBAL_USER }))
                    .try_into()?,
            ))
            .props(Some(
                CounterViewProps {
                    text: "The common counter".into(),
                }
                .try_into()?,
            ))
            .try_into()?,
    ])
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
