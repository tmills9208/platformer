use bevy::prelude::*;
use bevy::prelude::State;
// use bevy_parallax::{ParallaxResource, LayerData};

mod hello;
mod custom_parallax;
mod animated_sprite;

use crate::hello::HelloPlugin;
use crate::custom_parallax::CustomParallaxPlugin;
use crate::animated_sprite::AnimatedSpritePlugin;

fn main() {
    let window = WindowDescriptor {
        title: "Window name".to_string(),
        width: 1600.0,
        height: 900.0,
        resizable: false,
        ..Default::default()
    };

    /* In order, game will require:
    - Base assets & systems to load on startup
    - load & generate ui
    - start game in 'menu' gameState
    + world loading system (loading, entry/exit into other worlds/levels)
        - starts in the 'play' state, could reuse as a main menu background.
     */

    App::new()
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(CustomParallaxPlugin)
        .add_plugin(HelloPlugin)
        //.add_plugin(AnimatedSpritePlugin)
        .run();
}

struct Life(u8);

struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    life: Life,
    is_player: Player,
    #[bundle]
    entity_bundle: EntityBundle,
}

struct Position {
    x: f32,
    y: f32,
}

enum BaseEntityStates {
    Idle,
    Running,
    Walking,
    Attack,
    OnHit,
    Death
}

#[derive(Bundle)]
struct EntityBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    position: Position,
    animation_state: State<BaseEntityStates>
}

fn setup_entity() {

}
