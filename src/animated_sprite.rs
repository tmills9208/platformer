use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

#[derive(Component, Deref, DerefMut)]
struct Player {
    speed: f32,
}

#[derive(Component, Deref, DerefMut)]
struct SpriteMeta {
    flip: bool,
}

pub struct AnimatedSpritePlugin;

impl Plugin for AnimatedSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Transform::default())
            .add_system(animate_sprite)
            .add_system(move_sprite);
    }
}

pub struct KeyBinds {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        KeyBinds {
            move_left: KeyCode::A,
            move_right: KeyCode::D,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("npcs/Warrior_Sheet-Effect.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(69.0, 44.0), 6, 17);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(100.0, 100.0, 10.0).with_scale(Vec3::splat(2.0)),
            ..Default::default()
        })
        .insert(Player { speed: 200.0 })
        .insert(SpriteMeta { flip: false })
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn move_sprite(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player,
        &mut SpriteMeta,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
) {
    for (player, mut sprite_info, mut transform, mut sprite) in query.iter_mut() {
        // grab keybinds (unnecessary once I can load it as an asset)
        let keybinds = KeyBinds {
            ..Default::default()
        };

        // assign direction based on input
        let mut direction = 0.0;
        let mut flip = sprite_info.flip;
        if input.pressed(keybinds.move_left) {
            if !flip {
                flip = true;
            }
            direction -= 1.0;
        }
        if input.pressed(keybinds.move_right) {
            if flip {
                flip = false;
            }
            direction += 1.0;
        }
        sprite_info.flip = flip;

        // Calculate and apply movement
        let delta = time.delta_seconds() * player.speed;
        let translation = &mut transform.translation;

        sprite.flip_x = sprite_info.flip;
        translation.x += direction * delta;
    }
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let _texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            sprite.index = ((sprite.index + 1) % 8) + 6;
        }
    }
}
