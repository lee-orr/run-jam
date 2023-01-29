use belly::prelude::*;
use bevy::prelude::*;

use crate::level::GoalStatus;

#[allow(clippy::clone_on_copy)]
pub fn in_game_ui(mut commands: Commands) {
    commands.add(eml! {
        <body>
            <div c:score-container>
                <img bind:src=from!(GoalStatus:current.get_asset_string() | fmt.val("{val}"))/>
            </div>
        </body>
    });
}
