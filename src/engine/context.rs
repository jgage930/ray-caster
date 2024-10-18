use crate::engine::{cast::Ray, game_time::GameTimer, map::Map, player::Player};

#[derive(Debug)]
pub struct GameContext {
    pub map: Map,
    pub player: Player,
    pub rays: Option<Vec<Ray>>,
    pub time: GameTimer,
}

impl GameContext {
    pub fn update(&mut self) {
        let rotate_speed = 0.5;

        let delta = self.time.delta_time();
        self.player.looking_at += rotate_speed * delta;

        self.time.tick();
    }
}
