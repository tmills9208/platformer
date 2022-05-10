use bevy::prelude::*;

use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};

pub struct CustomParallaxPlugin;
impl Plugin for CustomParallaxPlugin {
    fn build(&self, app: &mut App) {
        let parallax = ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: 0.95,
                    speed_y: 1.0,
                    path: "parallax-mountain/parallax-mountain-bg.png".to_string(),
                    tile_size: Vec2::new(273.0, 160.0),

                    cols: 1,
                    rows: 1,
                    scale: 6.0,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.8,
                    speed_y: 0.8,
                    path: "parallax-mountain/parallax-mountain-montain-far.png".to_string(),
                    tile_size: Vec2::new(272.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 1.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.6,
                    speed_y: 0.6,
                    path: "parallax-mountain/parallax-mountain-mountains.png".to_string(),
                    tile_size: Vec2::new(544.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 2.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.4,
                    speed_y: 0.4,
                    path: "parallax-mountain/parallax-mountain-trees.png".to_string(),
                    tile_size: Vec2::new(544.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 3.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.2,
                    speed_y: 0.2,
                    path: "parallax-mountain/parallax-mountain-foreground-trees.png".to_string(),
                    tile_size: Vec2::new(544.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 4.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.3,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-1.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.1,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.32,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-2.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.2,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.34,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-3.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.3,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.36,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-4.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.4,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.38,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-5.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.5,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.4,
                    speed_y: 1.0,
                    path: "parallax-cube/parallax-cube-6.png".to_string(),
                    tile_size: Vec2::new(128.0, 128.0),
                    cols: 2,
                    rows: 3,
                    scale: 3.0,
                    z: 4.6,
                    ..Default::default()
                },
                
            ],
            ..Default::default()
        };

        app.insert_resource(parallax)
            .add_plugin(ParallaxPlugin)
            .add_startup_system(initialize_camera_system)
            .add_system(move_camera_system);
    }
}

fn initialize_camera_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(ParallaxCameraComponent);
}

fn move_camera_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);

    let shift = keyboard_input.pressed(KeyCode::LShift);

    let mut speed_boost = 1.0;
    let mut movement = Vec2::ZERO;

    if shift {
        speed_boost = 3.0;
    }

    if right {
        movement.x = 3.0 * speed_boost;
    } else if left {
        movement.x = -3.0 * speed_boost;
    }

    if up {
        movement.y = 1.0;
    } else if down {
        movement.y = -1.0;
    }

    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: movement.x,
        camera_move_speed_y: movement.y,
    });
}
