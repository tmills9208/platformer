use bevy::prelude::State;
use bevy::prelude::*;
// use bevy_ecs_ldtk::{LdtkPlugin};
// use bevy_parallax::{ParallaxResource, LayerData};

mod animated_sprite;
mod custom_parallax;
mod hello;

// use crate::animated_sprite::AnimatedSpritePlugin;
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
        columns: usize,
        rows: usize,
    ) -> Self {
        let texture_handle = asset_server.load(&url);
        let _texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(67.0, 78.0), columns, rows);
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
            .add_state(BaseEntityStates::Idle)
            .add_system(update_entity_frame);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_entity(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let animation_states: Vec<AnimationState> = vec![
        AnimationState {
            animation: Animation {
                name: "idle".to_string(),
                index_difference: 3,
                index_start: 56,
            },
            state: Some(BaseEntityStates::Idle),
        },
        AnimationState {
            animation: Animation {
                name: "walk".to_string(),
                index_difference: 4,
                index_start: 112,
            },
            state: Some(BaseEntityStates::Walking),
        },
        AnimationState {
            animation: Animation {
                name: "attack".to_string(),
                index_difference: 5,
                index_start: 21,
            },
            state: Some(BaseEntityStates::Attack),
        },
        AnimationState {
            animation: Animation {
                name: "on-hit".to_string(),
                index_difference: 3,
                index_start: 0,
            },
            state: Some(BaseEntityStates::OnHit),
        },
        AnimationState {
            animation: Animation {
                name: "death".to_string(),
                index_difference: 1,
                index_start: 31,
            },
            state: Some(BaseEntityStates::Death),
        },
    ];

    let _entity = commands
        .spawn_bundle(EntityBundle::from_asset(
            "MaplestoryDefaultSpriteSheet/maple-default.png".to_string(),
            asset_server,
            texture_atlas,
            10,
            13,
        ))
        .insert(EntityAnimations::setup(
            animation_states,
            Some(BaseEntityStates::Idle),
        ))
        .insert(AnimationTimer(Timer::from_seconds(0.4, true)))
        .insert(StateChangeTimer(Timer::from_seconds(2.0, true)))
        //.add_system()
        .id();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Animation {
    name: String,
    index_difference: usize,
    index_start: usize,
}

#[derive(Clone, PartialEq)]
struct AnimationState {
    animation: Animation,
    state: Option<BaseEntityStates>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
enum BaseEntityStates {
    Idle,
    Walking,
    Attack,
    OnHit,
    Death,
}

#[derive(Component)]
struct EntityAnimations {
    current_index: usize,
    animation_states: Vec<AnimationState>,
    current_state: AnimationState,
}

impl EntityAnimations {

    pub fn setup(
        animation_states: Vec<AnimationState>,
        mut current_state: Option<BaseEntityStates>,
    ) -> Self {
        if current_state.is_none() {
            current_state = Some(BaseEntityStates::Idle);
        }

        let _current_state = EntityAnimations::static_find_animation_state_by_state(
            animation_states.clone(),
            current_state.unwrap(),
        )
        .unwrap();

        let mut entity_animations = EntityAnimations {
            current_index: _current_state.animation.index_start,
            animation_states: animation_states.clone(),
            current_state: _current_state,
        };
        entity_animations.sort_animations();
        entity_animations.initialize_animation();

        entity_animations
    }

    pub fn update_animation(&mut self) {
        let current_state = self.current_state.clone();
        if current_state.state.is_none() {
            self.initialize_animation();
        }

        self.current_index += 1;
        if self.current_index
            > (current_state.animation.index_start + current_state.animation.index_difference)
        {
            let current_state = self
                .current_state
                .state
                .as_ref()
                .unwrap_or(&BaseEntityStates::Idle);
            let new_animation = &self
                .find_animation_state_by_state(current_state.clone())
                .unwrap();
            self.current_index = new_animation.animation.index_start;
        }
    }

    fn initialize_animation(&mut self) {
        let mut current_state = &self.current_state;
        if current_state.state.is_none() {
            self.current_state.state = Some(BaseEntityStates::Idle);
            current_state = &self.current_state;
            self.current_index = current_state.animation.index_start;
        }
    }

    fn find_animation_state_by_state(&self, state: BaseEntityStates) -> Option<AnimationState> {
        let animations = &self.animation_states;
        let mut result: Option<AnimationState> = None;
        for animation_state in animations.into_iter() {
            if animation_state.state == Some(state.clone()) {
                result = Some(animation_state.clone());
            }
        }
        result
    }

    pub fn static_find_animation_state_by_state(
        animation_states: Vec<AnimationState>,
        state: BaseEntityStates,
    ) -> Option<AnimationState> {
        let mut result: Option<AnimationState> = None;
        for animation_state in animation_states.into_iter() {
            if animation_state.state == Some(state.clone()) {
                result = Some(animation_state.clone());
            }
        }
        result
    }

    pub fn update_state(&mut self, new_entity_state: BaseEntityStates) {
        self.current_state.state = Some(new_entity_state);
    }

    pub fn insert_animation(
        &mut self,
        name: String,
        index_difference: usize,
        index_start: usize,
        state: BaseEntityStates,
    ) {
        let animation = Animation {
            name: name,
            index_difference: index_difference,
            index_start: index_start,
        };
        self.animation_states.push(AnimationState {
            state: Some(state),
            animation: animation,
        });
        self.sort_animations();
    }

    fn sort_animations(&mut self) {
        if &self.animation_states.len() > &1 {
            self.animation_states
                .sort_by(|a, b| a.animation.index_start.cmp(&b.animation.index_start));
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct StateChangeTimer(Timer);

fn update_entity_frame(
    time: Res<Time>,
    texture_atlas: Res<Assets<TextureAtlas>>,
    _state: Res<State<BaseEntityStates>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut EntityAnimations,
        &mut StateChangeTimer,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, mut entity_animations, mut state_timer, texture_atlas_handle) in
        query.iter_mut()
    {
        timer.tick(time.delta());
        if timer.just_finished() {
            let _texture_atlas = texture_atlas.get(texture_atlas_handle).unwrap();

            let new_index = entity_animations.current_index;
            entity_animations.update_animation();
            if entity_animations
                .current_state
                .state
                .as_ref()
                .clone()
                .eq(&Some(_state.current()))
            {
                entity_animations.update_state(_state.current().clone());
            }

            sprite.index = new_index;
        }

        state_timer.tick(time.delta());
        if state_timer.just_finished() {
            match _state.current() {
                BaseEntityStates::Idle => entity_animations.update_state(BaseEntityStates::Walking),
                BaseEntityStates::Walking => entity_animations.update_state(BaseEntityStates::Attack),
                BaseEntityStates::Attack => entity_animations.update_state(BaseEntityStates::OnHit),
                BaseEntityStates::OnHit => entity_animations.update_state(BaseEntityStates::Death),
                BaseEntityStates::Death => entity_animations.update_state(BaseEntityStates::Idle),
            }
        }
    }
}
