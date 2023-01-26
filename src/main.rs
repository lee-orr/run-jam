mod assets;
mod game_over_screen;
mod game_state;
mod goal;
mod gravity;
mod gravity_spawner;
mod in_game_ui;
mod level;
mod main_camera;
mod player;
mod space_material;

use std::time::Duration;

use assets::{GameAssets, GameLoadState};
use belly::prelude::StyleSheet;
use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use game_over_screen::setup_game_over;
use game_state::GameState;
use gravity::FIXED_TIME_MILIS;
use gravity_spawner::{Prediction, TrajectoryPoint};
use iyes_loopless::{
    prelude::{
        AppLooplessFixedTimestepExt, AppLooplessStateExt, ConditionSet, IntoConditionalSystem,
    },
    state::NextState,
};
use noisy_bevy::NoisyShaderPlugin;
use space_material::SpaceMaterial;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(67, 13, 75)))
        .add_loopless_state(GameLoadState::Loading)
        .add_loading_state(
            LoadingState::new(GameLoadState::Loading)
                .continue_to_state(GameLoadState::Ready)
                .with_collection::<GameAssets>(),
        )
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(belly::prelude::BellyPlugin)
        .add_plugin(Material2dPlugin::<space_material::SpaceMaterial>::default())
        .add_plugin(NoisyShaderPlugin);

    app.insert_resource(level::LevelBoundary {
        min: Vec2::new(-900., -500.),
        max: Vec2::new(900., 500.),
    })
    .insert_resource(goal::Score(0))
    .insert_resource(Prediction::None)
    .add_loopless_state(GameState::Loading)
    .add_startup_system(setup)
    .add_enter_system(GameLoadState::Ready, loaded)
    .add_enter_system(GameState::Playing, level::start_level)
    .add_enter_system(GameState::Playing, in_game_ui::in_game_ui)
    .add_exit_system(GameState::Playing, level::clear_level)
    .add_fixed_timestep(Duration::from_millis(FIXED_TIME_MILIS), "calculate_physics")
    .add_fixed_timestep_system_set(
        "calculate_physics",
        0,
        ConditionSet::new()
            .run_in_state(GameState::Playing)
            .with_system(gravity::calculate_gravity)
            .with_system(gravity::adjust_rotation)
            .into(),
    )
    .add_system_set(
        ConditionSet::new()
            .run_in_state(GameState::Playing)
            .with_system(main_camera::position_main_camera)
            .with_system(goal::check_goal)
            .with_system(gravity_spawner::gravity_spawner)
            .with_system(level::check_boundary)
            .with_system(level::update_backdrop)
            .with_system(gravity::smooth_movement)
            .with_system(gravity::predict_trajectory)
            .with_system(gravity::check_crash)
            .into(),
    )
    .add_enter_system(GameState::GameOver, setup_game_over)
    .add_exit_system(GameState::Playing, clear_ui)
    .add_exit_system(GameState::GameOver, clear_ui);

    app.run();
}

fn loaded(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}

fn clear_ui(mut commands: Commands, roots: Query<Entity, (With<Node>, Without<Parent>)>) {
    for root in roots.iter() {
        commands.entity(root).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut space_materials: ResMut<Assets<space_material::SpaceMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.add(StyleSheet::load("ui-style.ess"));
    commands
        .spawn((
            Camera2dBundle {
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedHorizontal(main_camera::BUFFER),
                    ..default()
                },
                ..default()
            },
            main_camera::MainCamera,
            main_camera::ElasticCentering(1., Vec2::ZERO),
            ComputedVisibility::default(),
            Visibility::default(),
        ))
        .with_children(|p| {
            p.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(2000., 4).into())
                    .into(),
                material: space_materials.add(SpaceMaterial::default()),
                transform: Transform::from_translation(Vec3::new(0., 0., -500.)),
                ..default()
            });
        });

    for _ in 0..10 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(3., 8).into()).into(),
                material: color_materials.add(Color::WHITE.into()),
                transform: Transform::default(),
                visibility: Visibility::INVISIBLE,
                ..default()
            },
            TrajectoryPoint,
        ));
    }
}
