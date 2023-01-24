mod goal;
mod gravity;
mod gravity_spawner;
mod level;
mod main_camera;
mod player;
mod space_material;

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};
use noisy_bevy::NoisyShaderPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(67, 13, 75)))
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
        .add_plugin(Material2dPlugin::<space_material::SpaceMaterial>::default())
        .add_plugin(NoisyShaderPlugin);

    app.insert_resource(level::LevelBoundary {
        min: Vec2::new(-900., -500.),
        max: Vec2::new(900., 500.),
    })
    .add_startup_system(setup)
    .add_system(gravity::calculate_gravity)
    .add_system(gravity::move_velocity)
    .add_system(goal::check_goal)
    .add_system(gravity_spawner::gravity_spawner)
    .add_system(main_camera::position_main_camera)
    .add_system(level::check_boundary);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut space_materials: ResMut<Assets<space_material::SpaceMaterial>>,
    asset_server: Res<AssetServer>,
    boundary: Res<level::LevelBoundary>,
) {
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
                material: space_materials.add(space_material::SpaceMaterial {
                    main_background: Color::rgb_u8(67, 13, 75),
                    highlight_color: Color::rgb_u8(204, 111, 218),
                    dark_color: Color::rgb_u8(23, 13, 25),
                    star_color: Color::rgb_u8(246, 225, 249),
                    map_boundary: Vec4::new(
                        boundary.min.x,
                        boundary.min.y,
                        boundary.max.x,
                        boundary.max.y,
                    ),
                }),
                transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
                ..default()
            });
        });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * 50.),
                ..Default::default()
            },
            texture: asset_server.load("large_planet.png"),
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        },
        gravity::GravitationalBody(10000., 50.),
        gravity::Velocity::Static,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * 50.),
                ..Default::default()
            },
            texture: asset_server.load("goal.png"),
            transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
            ..default()
        },
        goal::Goal(30.),
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE * 50.),
                ..Default::default()
            },
            texture: asset_server.load("player.png"),
            transform: Transform::from_translation(Vec3::new(-500., 0., 0.)),
            ..default()
        },
        gravity::GravitationalBody(1., 10.),
        player::Player,
        gravity::Velocity::Value(Vec2::X * 50.),
    ));
}
