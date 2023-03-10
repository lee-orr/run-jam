#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Menu,
    Credits,
    Playing,
    GameOver,
}
