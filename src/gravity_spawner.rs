use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::assets::GameAssets;
use crate::gravity::{self, GravitationTransform, GravitationalBody};
use crate::pickup::{ActivePickup, PickupType};
use crate::player::Player;

#[derive(Component)]
pub struct Deletable;

#[derive(Component)]
pub struct TrajectoryPoint;

#[derive(Resource)]
pub enum Prediction {
    None,
    Insert(Vec2, GravitationalBody),
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn gravity_spawner(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    existing_gravity: Query<Entity, With<Deletable>>,
    assets: Res<GameAssets>,
    prediction: Res<Prediction>,
    mut active_pickup: ResMut<ActivePickup>,
    players: Query<(Entity, &GravitationTransform, &Transform), With<Player>>,
) {
    let spawning = buttons.just_released(MouseButton::Left);
    let testing = buttons.pressed(MouseButton::Left);
    let initialized = buttons.just_pressed(MouseButton::Left);

    if (spawning || testing) && !initialized && matches!(*prediction, Prediction::None) {
        return;
    }

    if !spawning && !testing {
        return;
    }

    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    let screen_pos = wnd.cursor_position();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = screen_pos {
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

        let (mut possible_body, image) =
            (GravitationalBody(10000., 10.), assets.small_planet.clone());

        if matches!(active_pickup.0, Some(PickupType::Teleport)) {
            for (entity, player, transform) in players.iter() {
                if let GravitationTransform::Velocity {
                    velocity,
                    start_position: _,
                    target_position: _,
                } = player
                {
                    commands.entity(entity).insert((
                        transform.with_translation(Vec3::new(world_pos.x, world_pos.y, 0.)),
                        GravitationTransform::Velocity {
                            velocity: *velocity,
                            start_position: Some(world_pos),
                            target_position: Some(world_pos),
                        },
                    ));
                }
            }
            possible_body.0 = 0.;
        }

        if spawning && !matches!(*prediction, Prediction::None) {
            commands.insert_resource(Prediction::None);

            if matches!(active_pickup.0, Some(PickupType::Teleport)) {
                active_pickup.0 = None;
            } else {
                for entity in existing_gravity.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::ONE * 30.),
                            ..Default::default()
                        },
                        texture: image,
                        transform: Transform::from_translation(Vec3::new(
                            world_pos.x,
                            world_pos.y,
                            0.,
                        )),
                        ..default()
                    },
                    possible_body,
                    gravity::GravitationTransform::Static,
                    Deletable,
                    crate::level::LevelEntity,
                ));
            }
        } else {
            commands.insert_resource(Prediction::Insert(world_pos, possible_body));
        }
    } else {
        commands.insert_resource(Prediction::None);
    }
}
