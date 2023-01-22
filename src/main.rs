use bevy::{
    math::swizzles::*, prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle,
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
        .add_system(gravity_spawner);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PINK)),
            transform: Transform::from_translation(Vec3::new(0., -100., 0.)),
            ..default()
        },
        GravitationalBody(10000.),
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
        GravitationalBody(1.),
        Player,
        Velocity::Value(Vec2::X * 50.),
    ));
}

#[derive(Component)]
pub struct GravitationalBody(f32);

#[derive(Component)]
pub enum Velocity {
    Static,
    Value(Vec2),
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Goal(f32);

const G: f32 = 30.;

fn calculate_gravity(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Velocity, &GravitationalBody)>,
    time: Res<Time>,
) {
    for (entity, transform, velocity, gravity) in query.iter() {
        if let Velocity::Value(v) = velocity {
            let mut new_v = v.clone();
            for (entity_2, t_2, _, g_2) in query.iter() {
                if entity_2 == entity {
                    continue;
                }
                let r = (t_2.translation - transform.translation).xy();
                info!("Adjusting velocity {new_v:?}");
                let d_sq = r.length_squared();
                if d_sq != 0. {
                    new_v +=
                        (G * time.delta_seconds() * gravity.0 * g_2.0 * r.normalize()) / (d_sq);
                }
                info!("into {new_v:?}");
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
            info!("Moving {displacement:?}");
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
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PINK)),
                transform: Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, 0.)),
                ..default()
            },
            GravitationalBody(10000.),
            Velocity::Static,
        ));
    }
}
