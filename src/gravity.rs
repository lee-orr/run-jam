use bevy::{math::Vec3Swizzles, prelude::*};

#[derive(Component)]
pub struct GravitationalBody(pub f32, pub f32);

#[derive(Component)]
pub enum Velocity {
    Static,
    Value(Vec2),
}

pub(crate) const G: f32 = 30.;

pub(crate) fn calculate_gravity(
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

pub(crate) fn move_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
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
