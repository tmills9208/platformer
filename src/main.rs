use bevy::prelude::State;
use bevy::prelude::*;
// use bevy_ecs_ldtk::{LdtkPlugin};
// use bevy_parallax::{ParallaxResource, LayerData};

mod animated_sprite;
mod custom_parallax;
mod hello;

use crate::animated_sprite::AnimatedSpritePlugin;
use crate::custom_parallax::CustomParallaxPlugin;
use crate::hello::HelloPlugin;

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
        //.add_plugin(LdtkPlugin)
        .add_plugin(CustomParallaxPlugin)
        .add_plugin(HelloPlugin)
        .add_plugin(CharacterPlugin)
        //.add_plugin(AnimatedSpritePlugin)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Life(u8);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    life: Life,
    is_player: Player,
    #[bundle]
    entity_bundle: EntityBundle,
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Deref)]
struct SpriteSheetURL(String);

#[derive(Bundle)]
struct EntityBundle {
    sheet_url: SpriteSheetURL,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    position: Position,
}

impl EntityBundle {
    pub fn from_asset(
        url: String,
        asset_server: Res<AssetServer>,
        mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load(&url);
        let _texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(67.0, 78.0), 10, 9);
        let texture_atlas_handle = texture_atlas.add(_texture_atlas);

        EntityBundle {
            sheet_url: SpriteSheetURL(url.to_string()),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(100.0, 100.0, 100.0),
                ..Default::default()
            },
            position: Position { x: 100.0, y: 100.0 },
        }
    }
}

struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_entity)
            .add_state(BaseEntityStates::Idle);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let _entity = commands
        .spawn_bundle(EntityBundle::from_asset(
            "MaplestoryDefaultSpriteSheet/maple-default-sheet.png".to_string(),
            asset_server,
            texture_atlas,
        ))
        .id();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum BaseEntityStates {
    Idle,
    Walking,
    Attack,
    OnHit,
    Death,
}


