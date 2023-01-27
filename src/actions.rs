use bevy::prelude::*;
use iyes_loopless::state::NextState;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Action {
    GravityWell,
    PortableHole,
    GravityInverter,
}

#[derive(Resource, Default, Debug)]
pub struct AvailableActions {
    pub portable_hole: usize,
    pub gravity_inverter: usize,
}

pub fn set_action(
    mut commands: Commands,
    mut event: EventReader<Action>,
    mut available: ResMut<AvailableActions>,
) {
    if let Some(event) = event.iter().next() {
        let valid = match event {
            Action::GravityWell => true,
            Action::PortableHole => {
                if available.portable_hole > 0 {
                    available.portable_hole -= 1;
                    true
                } else {
                    false
                }
            }
            Action::GravityInverter => {
                if available.gravity_inverter > 0 {
                    available.gravity_inverter -= 1;
                    true
                } else {
                    false
                }
            }
        };
        if valid {
            commands.insert_resource(NextState(event.clone()));
        }
    }
}
