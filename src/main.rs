use lenra_app::{manifest::Manifest, LenraApp, resource::map_resources};
use resources::RESOURCE_MAP;
mod data;
mod listeners;
mod resources;
mod views;

fn main() {
    let app = LenraApp {
        manifest: Manifest {
            root_view: "main".into(),
        },
        views: vec![],
        listeners: vec![],
        resources: map_resources(RESOURCE_MAP),
        ..Default::default()
    };

    app.run().unwrap();
}
