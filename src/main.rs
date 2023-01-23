use bevy::{
    math::swizzles::*,
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
    sprite::MaterialMesh2dBundle,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(67, 13, 75)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }));

    app.add_startup_system(setup)
        .add_system(calculate_gravity)
        .add_system(move_velocity)
        .add_system(check_goal)
        .add_system(gravity_spawner)
        .add_system(position_main_camera);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(BUFFER),
                ..default()
            },
            ..default()
        },
        MainCamera,
        ElasticCentering(1., Vec2::ZERO),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PINK)),
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        },
        GravitationalBody(10000., 50.),
        Velocity::Static,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(0., 50., 0.)),
            ..default()
        },
        Goal(30.),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
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

fn gravity_spawner(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_gravity: Query<(Entity, &GlobalTransform, &GravitationalBody), With<Deletable>>,
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
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PINK)),
                    transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.)),
                    ..default()
                },
                GravitationalBody(10000., 50.),
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
