use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    actions::{Action, AvailableActions, NextAction},
    gravity_spawner::Prediction,
    pickup::Score,
};

#[allow(clippy::clone_on_copy)]
pub fn in_game_ui(mut commands: Commands) {
    commands.add(eml! {
        <body>
            <div c:score-container>
                {from!(Score:value() | fmt.s("{s:0}"))}
            </div>
            <div c:actions>
                <button c:action_button on:press=connect!(|ctx| {
                    ctx.commands().insert_resource(Prediction::None);
                    ctx.commands().insert_resource(NextAction(Action::GravityInverter));
                })><div c:content>{from!(AvailableActions:inverter_button_display() | fmt.s("{s}"))}</div></button>
                <button c:action_button on:press=connect!(|ctx| {
                    ctx.commands().insert_resource(Prediction::None);
                    ctx.commands().insert_resource(NextAction(Action::PortableHole));
                })><div c:content>{from!(AvailableActions:hole_button_display() | fmt.s("{s}"))}</div></button>
            </div>
        </body>
    });
}
