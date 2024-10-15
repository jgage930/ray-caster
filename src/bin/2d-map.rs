use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use ray_caster::engine::context::GameContext;
use ray_caster::engine::map::Map;
use ray_caster::engine::renderer::Renderer;

fn main() {
    let map = Map::new("maps/test_map.txt").expect("Could not create map.");

    let context = GameContext { map };

    let window_width = (map.width() * map.tile_size()) as u32;
    let window_height = (map.height() * map.tile_size()) as u32;

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
        renderer.update().expect("Failed to update Game Loop.");

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
