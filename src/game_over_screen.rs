use belly::prelude::*;
use bevy::prelude::*;

use crate::{game_state::*, level::GoalStatus};
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_game_over(mut commands: Commands, goals: Res<GoalStatus>) {
    let goals = goals
        .completed
        .iter()
        .map(|g| g.get_asset_string())
        .collect::<Vec<_>>();
    commands.add(eml! {
        <body>
            <div c:modal>
                <div c:modal_content>
                <div c:header>
                    <for goal in = goals>
                        <img src=goal/>
                    </for>
                </div>
                <div>
                    <button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Playing)))>
                        <span c:content>
                            <img src="paper.png"/>
                        </span>
                    </button>
                    <button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Menu)))>
                        <span c:content>
                            <img src="credits.png"/>
                        </span>
                    </button>
                </div>
                </div>
            </div>
        </body>
    });
}
