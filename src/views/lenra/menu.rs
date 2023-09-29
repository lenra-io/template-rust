use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};

pub fn menu(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = container()
        .decoration(Some(
            StylesBoxDecoration::builder()
                .color(StylesColor(0xFFFFFFFF))
                .box_shadow(Some(
                    StylesBoxShadow::builder()
                        .blur_radius(8_f64)
                        .color(StylesColor(0x1A000000))
                        .offset(Some(
                            StylesOffset::builder().dx(0_f64).dy(1_f64).try_into()?,
                        ))
                        .try_into()?,
                ))
                .try_into()?,
        ))
        .padding(StylesPadding::symmetric(16, 32))
        .child(Some(menu_content()?.try_into()?))
        .into();

    Ok(result.gen())
}

fn menu_content() -> Result<LenraComponent> {
    Ok(flex(vec![
        container()
            .constraints(Some(
                StylesBoxConstraints::builder()
                    .min_width(32_f64)
                    .min_height(32_f64)
                    .max_width(32_f64)
                    .max_height(32_f64)
                    .try_into()?,
            ))
            .child(Some(Box::new(image("logo.png").into())))
            .into(),
        flexible(Box::new(
            container()
                .child(Some(Box::new(
                    text("Hello World")
                        .text_align(TextTextAlign::Center)
                        .style(Some(
                            StylesTextStyle::builder()
                                .font_weight(StylesTextStyleFontWeight::Bold)
                                .font_size(24_f64)
                                .try_into()?,
                        ))
                        .into(),
                )))
                .into(),
        ))
        .into(),
    ])
    .fill_parent(true)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceBetween)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .padding(Some(StylesPadding::builder().right(32_f64).try_into()?))
    .into())
}
