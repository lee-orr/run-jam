use belly::prelude::*;
use bevy::prelude::*;

use crate::{game_state::*, pickup::Score};
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    let score = score.0.to_string();
    commands.add(eml! {
        <body>
            <div c:modal>
                <div c:modal_content>
                <div c:header>
                    {score}
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
