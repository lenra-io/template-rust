use lenra_app::{
    components::lenra::{view, ViewDefinitionsFind},
    manifest::{Exposer, Manifest, Route},
    resource::map_resources,
    LenraApp, Result,
};
use listeners::{get_listeners, COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER};
use resources::RESOURCE_MAP;
use serde_json::json;
use views::get_views;
mod data;
mod listeners;
mod resources;
mod views;

fn main() -> Result<()> {
    let app = LenraApp {
        manifest: Manifest::builder()
            .json(Some(
                Exposer::builder()
                    .routes(vec![
                        Route::builder()
                            .path("/counter/global")
                            .view(
                                view("json:counter").find(Some(
                                    ViewDefinitionsFind::builder()
                                        .coll(COUNTER_COLLECTION)
                                        .query(json!({ "user": GLOBAL_USER }))
                                        .try_into()?,
                                )),
                            )
                            .try_into()
                            .unwrap(),
                        Route::builder()
                            .path("/counter/me")
                            .view(
                                view("json:counter").find(Some(
                                    ViewDefinitionsFind::builder()
                                        .coll(COUNTER_COLLECTION)
                                        .query(json!({ "user": CURRENT_USER }))
                                        .try_into()?,
                                )),
                            )
                            .try_into()
                            .unwrap(),
                    ])
                    .try_into()?,
            ))
            .lenra(Some(
                Exposer::builder()
                    .routes(vec![Route::builder()
                        .path("/")
                        .view(view("lenra:main"))
                        .try_into()
                        .unwrap()])
                    .try_into()
                    .unwrap(),
            ))
            .try_into()
            .unwrap(),
        views: get_views(),
        listeners: get_listeners(),
        resources: map_resources(RESOURCE_MAP),
    };

    app.run()
}
