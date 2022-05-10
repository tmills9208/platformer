use bevy::prelude::*;

mod hello;
mod parallax;

use crate::hello::HelloPlugin;
use crate::parallax::CustomParallaxPlugin;

fn main() {
    let window = WindowDescriptor {
        title: "Window name".to_string(),
        width: 1600.0,
        height: 900.0,
        resizable: false,
        ..Default::default()
    };

    App::new()
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomParallaxPlugin)
        .add_plugin(HelloPlugin)
        .run();
}
