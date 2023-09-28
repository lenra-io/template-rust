use lenra_app::{
    components::lenra::{flex, view, FlexCrossAxisAlignment, LenraComponent, StylesDirection},
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};

pub fn main(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![view("lenra:menu").into(), view("lenra:home").into()])
        .direction(StylesDirection::Vertical)
        .scroll(true)
        .spacing(4_f64)
        .cross_axis_alignment(FlexCrossAxisAlignment::Center)
        .into();
    Ok(result.gen())
}
