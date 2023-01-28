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
    #[asset(path = "hole_pickup.png")]
    pub hole_pickup: Handle<Image>,
    #[asset(path = "hole.png")]
    pub hole: Handle<Image>,
    #[asset(path = "inverter_pickup.png")]
    pub inverter_pickup: Handle<Image>,
    #[asset(path = "inverter.png")]
    pub inverter: Handle<Image>,

    #[asset(path = "chips.png")]
    pub chips: Handle<Image>,
    #[asset(path = "fruit.png")]
    pub fruit: Handle<Image>,
    #[asset(path = "gas.png")]
    pub gas: Handle<Image>,
    #[asset(path = "post.png")]
    pub post: Handle<Image>,
    #[asset(path = "toilet-paper.png")]
    pub toilet_paper: Handle<Image>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameLoadState {
    Loading,
    Ready,
}
