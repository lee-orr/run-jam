use bevy::prelude::*;
use iyes_loopless::state::NextState;

use crate::level::LevelEvent;

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

#[derive(Resource)]
pub struct NextAction(pub Action);

pub fn set_action(
    mut commands: Commands,
    action: Res<NextAction>,
    mut available: ResMut<AvailableActions>,
) {
    if action.is_changed() {
        return;
    }
    let valid = match action.0 {
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
        commands.insert_resource(NextState(action.0.clone()));
    }
}

impl AvailableActions {
    pub fn hole_button_display(&self) -> String {
        if self.portable_hole > 0 {
            format!("{} Portable Holes", self.portable_hole)
        } else {
            "No Holes".to_string()
        }
    }
    pub fn inverter_button_display(&self) -> String {
        if self.gravity_inverter > 0 {
            format!("{} Inverters", self.gravity_inverter)
        } else {
            "No Inverters".to_string()
        }
    }
}

pub fn pickup_action_collected(
    mut available: ResMut<AvailableActions>,
    mut event: EventReader<LevelEvent>,
) {
    for event in event.iter() {
        match event {
            LevelEvent::PickupCollected(p) => match p {
                crate::pickup::PickupType::Hole => {
                    available.portable_hole += 1;
                }
                crate::pickup::PickupType::Inverter => {
                    available.gravity_inverter += 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
