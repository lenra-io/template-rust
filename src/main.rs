use lenra_app::{manifest::Manifest, resource::map_resources, LenraApp};
use listeners::get_listeners;
use resources::RESOURCE_MAP;
use views::get_views;
mod data;
mod listeners;
mod resources;
mod views;

fn main() {
    let app = LenraApp {
        manifest: Manifest {
            root_view: "main".into(),
        },
        views: get_views(),
        listeners: get_listeners(),
        resources: map_resources(RESOURCE_MAP),
        ..Default::default()
    };

    app.run().unwrap();
}
