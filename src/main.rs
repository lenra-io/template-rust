use lenra_app::{resource::map_resources, LenraApp, Result};
use listeners::get_listeners;
use manifest::get_manifest;
use resources::RESOURCE_MAP;
use views::get_views;

mod data;
mod listeners;
mod manifest;
mod resources;
mod views;

fn main() -> Result<()> {
    let app = LenraApp {
        manifest: get_manifest()?,
        views: get_views(),
        listeners: get_listeners(),
        resources: map_resources(RESOURCE_MAP),
    };

    app.run()
}
