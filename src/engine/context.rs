use crate::{engine::map::Map, engine::player::Player};

#[derive(Debug)]
pub struct GameContext {
    pub map: Map,
    pub player: Player,
}

