use ray_caster::map::*;
use ray_caster::player::*;
use ray_caster::*;

fn main() {
    let map_file = "
        1111111111
        1        1
        1  1111111
        1        1
        1    11111
        1    1   1
        1   1    1
        1        1
        1        1
        1111111111
    "
    .trim();

    let map = Map::new(64, map_file);
    let mut buf = map.into_buffer();

    buf.set_draw_color(Color::GREEN);
    buf.draw(&map);

    let player = Player::new(100, 100);

    buf.set_draw_color(Color::RED);
    buf.draw(&player);

    player.cast_ray(&map, &mut buf);

    save_ppm("images/player.ppm", &buf).expect("Could not write to file.");
}
