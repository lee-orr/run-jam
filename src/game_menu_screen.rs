use belly::prelude::*;
use bevy::prelude::*;

use crate::game_state::*;
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_menu(mut commands: Commands) {
    commands.add(eml! {
        <body>
        <div c:menu_image>
            <img src="menu-image.png"/>
        </div>
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
                        <button c:credits_button on:press=connect!(|ctx| ctx.commands().insert_resource(NextState(GameState::Credits)))>
                            <span c:content>
                                "Credits"
                            </span>
                        </button>
                    </div>
                </div>
            </div>
        </body>
    });
}
