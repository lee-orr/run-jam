#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Playing,
    GameOver,
}
