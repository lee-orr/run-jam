use belly::prelude::*;
use bevy::prelude::*;

use crate::{game_state::*, goal::Score};
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    let score = score.0.to_string();
    commands.add(eml! {
        <body>
            <div c:modal>
                <div c:modal_content>
                <div c:header>
                    "Got "{score}" Points"
                </div>
                <div>
                    <button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Playing)))><span c:content>"Restart"</span></button>
                </div>
                </div>
            </div>
        </body>
    });
}
