use lenra_app::view::View;

mod json;
mod lenra;

pub fn get_views() -> Vec<View> {
    let mut views: Vec<View> = vec![];
    views.append(&mut json::get_views());
    views.append(&mut lenra::get_views());
    views
}
