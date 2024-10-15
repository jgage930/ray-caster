use crate::{engine::map::Map, engine::player::Player};

use super::cast::Ray;

#[derive(Debug)]
pub struct GameContext {
    pub map: Map,
    pub player: Player,
    pub rays: Option<Vec<Ray>>,
}

