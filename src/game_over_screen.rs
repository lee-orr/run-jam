use belly::prelude::*;
use bevy::prelude::*;

use crate::game_state::*;
use iyes_loopless::prelude::*;

pub fn setup_game_over(mut commands: Commands) {
    commands.add(eml! {
        <body>
            <div c:modal>
                <div c:modal_content>
                <div c:header>
                    "Game Over!"
                </div>
                <div>
                    <button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Playing)))><span c:content>"Restart"</span></button>
                </div>
                </div>
            </div>
        </body>
    });
}
