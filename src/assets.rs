use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "player.png")]
    pub player: Handle<Image>,
    #[asset(path = "large_planet.png")]
    pub large_planet: Handle<Image>,
    #[asset(path = "small_planet.png")]
    pub small_planet: Handle<Image>,
    #[asset(path = "goal.png")]
    pub goal: Handle<Image>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameLoadState {
    Loading,
    Ready,
}
