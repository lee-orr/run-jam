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
    #[asset(path = "planet_killer_pickup.png")]
    pub planet_killer_pickup: Handle<Image>,
    #[asset(path = "player_planet_killer.png")]
    pub player_planet_killer: Handle<Image>,
    #[asset(path = "teleport.png")]
    pub teleport: Handle<Image>,
    #[asset(path = "player_teleport.png")]
    pub player_teleport: Handle<Image>,

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

    #[asset(path = "menu-image.png")]
    pub menu_image: Handle<Image>,

    #[asset(path = "galactic-errands-background.mp3")]
    pub music: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "collected.mp3")]
    pub collected_audio: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "pickup.mp3")]
    pub pickup_audio: Handle<bevy_kira_audio::AudioSource>,
    #[asset(path = "destroyed.mp3")]
    pub destroyed_audio: Handle<bevy_kira_audio::AudioSource>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameLoadState {
    Loading,
    Ready,
}
