use belly::prelude::*;
use bevy::prelude::*;

use crate::game_state::*;
use iyes_loopless::prelude::*;

#[allow(clippy::clone_on_copy)]
pub fn setup_credits(mut commands: Commands) {
    commands.add(eml! {
        <body>
        <div c:menu_image>
            <img src="menu-image.png"/>
        </div>
            <div c:menu>
                <div>
                    <div c:header>
                        "Credits"
                    </div>
                    <div c:subheader>
                        "Game concept, code, music & art assets created by Lee-Orr"
                    </div>
                    <div c:subheader>
                        "Using the following rust crates:"
                    </div>
                    <div>
                        "The Bevy Game Engine - bevyengine.org"
                    </div>
                    <div>
                        "Noise Bevy - https://github.com/johanhelsing/noisy_bevy"
                    </div>
                    <div>
                        "Bevy Asset Loader - https://github.com/NiklasEi/bevy_asset_loader"
                    </div>
                    <div>
                        "Iyes Loopless - https://github.com/IyesGames/iyes_loopless"
                    </div>
                    <div>
                        "Belly - https://github.com/jkb0o/belly"
                    </div>
                    <div>
                        "Bevy Turborand - https://github.com/Bluefinger/bevy_turborand"
                    </div>
                    <div>
                        "Bevy Kira Audio - https://github.com/NiklasEi/bevy_kira_audio"
                    </div>
                    <div c:buttons>
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
