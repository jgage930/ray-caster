pub mod engine;
pub mod utils;

use sdl2::{event::Event, keyboard::Keycode};

use std::time::Duration;

use engine::{
    cast::cast_rays, context::GameContext, game_time::GameTimer, map::Map, player::Player,
    renderer::Renderer,
};

fn main() {
    let map = Map::new("maps/test_map.txt").expect("Could not create map.");
    let player = Player::new(100, 100);
    let time = GameTimer::new();

    let mut context = GameContext {
        map: map.clone(),
        player,
        rays: None,
        time,
    };

    let window_width = 512;
    let window_height = 640;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("2d Map", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = Renderer::new(window).expect("Faile to create renderer from window.");

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        renderer
            .render_3d(512, &context)
            .expect("Failed to update Game Loop.");

        let ray = cast_rays(&context);
        context.rays = Some(ray);

        context.update();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
