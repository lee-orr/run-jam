use belly::prelude::*;
use bevy::prelude::*;

use crate::goal::Score;

#[allow(clippy::clone_on_copy)]
pub fn in_game_ui(mut commands: Commands) {
    commands.add(eml! {
        <body>
            <div c:score-container>
                {from!(Score:value() | fmt.s("{s:0}"))}
            </div>
        </body>
    });
}
