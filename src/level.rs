use crate::player;
use bevy::{math::Vec3Swizzles, prelude::*};



#[derive(Resource)]
pub struct LevelBoundary {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

pub(crate) fn check_boundary(
    players: Query<&Transform, With<player::Player>>,
    boundary: Res<LevelBoundary>,
) {
    for player in players.iter() {
        if player.translation.xy().cmplt(boundary.min).any()
            || player.translation.xy().cmpgt(boundary.max).any()
        {
            println!("Boundary Crossed! HELP!");
        }
    }
}
