use belly::prelude::*;
use bevy::prelude::*;

use crate::{game_state::*, pickup::Score};
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_menu(mut commands: Commands, score: Res<Score>) {
    let _score = score.0.to_string();
    commands.add(eml! {
        <body>
            <div c:menu>
                <div>
                    <div c:header>
                        "Galactic Errands"
                    </div>
                    <div c:subheader>
                        "By Lee-Orr"
                    </div>
                    <div c:buttons>
                        <button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Playing)))>
                            <span c:content>
                                <img src="paper.png"/>
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        </body>
    });
}
