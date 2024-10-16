use crate::{engine::map::Map, engine::player::Player};

use super::cast::Ray;

#[derive(Debug)]
pub struct GameContext {
    pub map: Map,
    pub player: Player,
    pub rays: Option<Vec<Ray>>,
}

impl GameContext {
    pub fn update(&mut self) {
        let rotate_speed = 0.1;

        self.player.looking_at += rotate_speed;
    }
}

