use ray_caster::map::*;
use ray_caster::*;

fn main() {
    let map_file = "
        1111111111
        1        1
        1  1     1
        1        1
        1    1   1
        1    1   1
        1    1   1
        1        1
        1        1
        1111111111
    "
    .trim();
    let map = Map::new(64, map_file);

    let mut buf = map.into_buffer();

    buf.set_draw_color(Color::GREEN);
    buf.draw(map);

    save_ppm("images/map.ppm", &buf).expect("Could not write to file.");
}
