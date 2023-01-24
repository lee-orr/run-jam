use bevy::{
    math::swizzles::*,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{RenderTarget, ScalingMode},
        render_resource::AsBindGroup,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}, log::Level,
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
        .add_plugin(Material2dPlugin::<SpaceMaterial>::default())
        .add_plugin(NoisyShaderPlugin);

    app
        .insert_resource(LevelBoundary {
            min: Vec2::new(-900., -500.),
            max: Vec2::new(900., 500.)
        })
        .add_startup_system(setup)
        .add_system(calculate_gravity)
        .add_system(move_velocity)
        .add_system(check_goal)
        .add_system(gravity_spawner)
        .add_system(position_main_camera)
        .add_system(check_boundary);

    app.run();
}

#[derive(Resource)]
pub struct LevelBoundary { min: Vec2, max: Vec2}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut space_materials: ResMut<Assets<SpaceMaterial>>,
    asset_server: Res<AssetServer>,
    boundary: Res<LevelBoundary>
) {
    commands
        .spawn((
            Camera2dBundle {
                projection: OrthographicProjection {
                    scaling_mode: ScalingMode::FixedHorizontal(BUFFER),
                    ..default()
                },
                ..default()
            },
            MainCamera,
            ElasticCentering(1., Vec2::ZERO),
            ComputedVisibility::default(),
            Visibility::default(),
        ))
        .with_children(|p| {
            p.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(2000., 4).into())
                    .into(),
                material: space_materials.add(SpaceMaterial {
                    main_background: Color::rgb_u8(67, 13, 75),
                    highlight_color: Color::rgb_u8(204, 111, 218),
                    dark_color: Color::rgb_u8(23, 13, 25),
                    star_color: Color::rgb_u8(246, 225, 249),
                    map_boundary: Vec4::new(boundary.min.x, boundary.min.y, boundary.max.x, boundary.max.y),
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
        GravitationalBody(10000., 50.),
        Velocity::Static,
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
        Goal(30.),
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
        GravitationalBody(1., 10.),
        Player,
        Velocity::Value(Vec2::X * 50.),
    ));
}

#[derive(Component)]
pub struct GravitationalBody(f32, f32);

#[derive(Component)]
pub enum Velocity {
    Static,
    Value(Vec2),
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Goal(f32);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ElasticCentering(f32, Vec2);

#[derive(Component)]
pub struct Deletable;

const G: f32 = 30.;

fn calculate_gravity(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Velocity, &GravitationalBody)>,
    time: Res<Time>,
) {
    for (entity, transform, velocity, gravity) in query.iter() {
        if let Velocity::Value(v) = velocity {
            let mut new_v = *v;
            for (entity_2, t_2, _, g_2) in query.iter() {
                if entity_2 == entity {
                    continue;
                }
                let r = (t_2.translation - transform.translation).xy();
                let d_sq = r.length_squared();
                if d_sq > 30. {
                    new_v +=
                        (G * time.delta_seconds() * gravity.0 * g_2.0 * r.normalize()) / (d_sq);
                }
            }
            commands.entity(entity).insert(Velocity::Value(new_v));
        }
    }
}

fn move_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, v) in query.iter_mut() {
        if let Velocity::Value(v) = v {
            let translation = transform.translation;
            let displacement = *v * time.delta_seconds();
            let translation = translation + Vec3::new(displacement.x, displacement.y, 0.);
            let angle = v.y.atan2(v.x);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
            transform.translation = translation;
        }
    }
}

fn check_goal(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    goals: Query<(Entity, &Transform, &Goal)>,
) {
    for player in players.iter() {
        for (entity, goal_transform, goal_radius) in goals.iter() {
            if player.translation.distance(goal_transform.translation) <= goal_radius.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn check_boundary(players: Query<&Transform, With<Player>>, boundary: Res<LevelBoundary>) {
    for player in players.iter() {
        if player.translation.xy().cmplt(boundary.min).any() || player.translation.xy().cmpgt(boundary.max).any() {
            println!("Boundary Crossed! HELP!");
        }
    }
}

fn gravity_spawner(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    existing_gravity: Query<(Entity, &GlobalTransform, &GravitationalBody), With<Deletable>>,
    asset_server: Res<AssetServer>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        let mut deleted = false;

        for (entity, transform, gravity) in existing_gravity.iter() {
            if transform.translation().xy().distance(world_pos) < gravity.1 {
                deleted = true;
                commands.entity(entity).despawn_recursive();
            }
        }

        if !deleted {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::ONE * 30.),
                        ..Default::default()
                    },
                    texture: asset_server.load("small_planet.png"),
                    transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.)),
                    ..default()
                },
                GravitationalBody(10000., 30.),
                Velocity::Static,
                Deletable,
            ));
        }
    }
}

const BUFFER: f32 = 1200.0;
const VELOCITY_RATIO: f32 = 5.;
const VELOCITY_OFFSET_MAX: Vec2 = Vec2::splat(300.);
const VELOCITY_OFFSET_MIN: Vec2 = Vec2::splat(-300.);

type ElasticCamera<'a> = (
    &'a mut Transform,
    &'a mut OrthographicProjection,
    &'a mut ElasticCentering,
);

fn position_main_camera(
    mut camera: Query<ElasticCamera, (With<MainCamera>, Without<Player>)>,
    players: Query<(&Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    let mut player_bounds = None;
    let mut target_offset = Vec2::ZERO;
    let mut num_players = 0.;
    for (player, velocity) in players.iter() {
        num_players += 1.;
        let pos = player.translation.xy();
        let offset = match velocity {
            Velocity::Static => Vec2::ZERO,
            Velocity::Value(v) => *v * VELOCITY_RATIO,
        };
        let offset = offset.min(VELOCITY_OFFSET_MAX).max(VELOCITY_OFFSET_MIN);
        target_offset += offset;

        if let Some((min, max)) = player_bounds {
            player_bounds = Some((pos.min(min), pos.max(max)));
        } else {
            player_bounds = Some((pos, pos));
        }
    }

    if num_players > 0. {
        target_offset /= num_players;
    }

    let delta = time.delta_seconds();

    for (mut transform, mut projection, mut centering) in camera.iter_mut() {
        let camera_center = transform.translation.xy();

        let (gap, pos) = if let Some((min, max)) = player_bounds {
            let gap = (max - min) / 2.;
            (gap, gap + min)
        } else {
            (Vec2::ZERO, Vec2::ZERO)
        };

        let max_component = gap.max_element();
        let horizontal = BUFFER + max_component;

        let offset = (target_offset * delta + centering.1 * (centering.0 - delta)) / centering.0;
        centering.1 = offset;

        let pos = pos + offset;
        let target_dist = (pos - camera_center) * 2.;
        let pos = target_dist * delta + camera_center;
        transform.translation = Vec3::new(pos.x, pos.y, 0.);

        projection.scaling_mode = ScalingMode::FixedHorizontal(horizontal);
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "6d535a38-2b0f-4d43-9bc2-2f000a2c9b33"]
pub struct SpaceMaterial {
    #[uniform(0)]
    main_background: Color,
    #[uniform(1)]
    highlight_color: Color,
    #[uniform(2)]
    dark_color: Color,
    #[uniform(3)]
    star_color: Color,
    #[uniform(4)]
    map_boundary: Vec4,
}

impl Material2d for SpaceMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "space.wgsl".into()
    }
}
