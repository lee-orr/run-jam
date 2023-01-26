use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::assets::GameAssets;
use crate::gravity::{self, GravitationalBody};

#[derive(Component)]
pub struct Deletable;

#[derive(Component)]
pub struct TrajectoryPoint;

#[derive(Resource)]
pub enum Prediction {
    None,
    Delete(Entity),
    Insert(Vec2, GravitationalBody),
}

pub(crate) fn gravity_spawner(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    existing_gravity: Query<
        (Entity, &GlobalTransform, &gravity::GravitationalBody),
        With<Deletable>,
    >,
    assets: Res<GameAssets>,
) {
    let spawning = buttons.just_released(MouseButton::Left);
    let testing = buttons.pressed(MouseButton::Left);
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

        let mut delete = None;

        for (entity, transform, gravity) in existing_gravity.iter() {
            if transform.translation().xy().distance(world_pos) < gravity.1 {
                delete = Some(entity);
                break;
            }
        }

        if spawning {
            commands.insert_resource(Prediction::None);
            match delete {
                Some(delete) => {
                    commands.entity(delete).despawn_recursive();
                }
                None => {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::ONE * 30.),
                                ..Default::default()
                            },
                            texture: assets.small_planet.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                world_pos.x,
                                world_pos.y,
                                0.,
                            )),
                            ..default()
                        },
                        gravity::GravitationalBody(10000., 30.),
                        gravity::GravitationTransform::Static,
                        Deletable,
                    ));
                }
            }
        } else {
            let prediction = match delete {
                Some(delete) => Prediction::Delete(delete),
                None => Prediction::Insert(world_pos, GravitationalBody(10000., 30.)),
            };
            commands.insert_resource(prediction);
        }
    }
}
