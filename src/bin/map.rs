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

    // let mut buf = FrameBuffer::new(500, 500, &Color::BLACK);
    buf.set_draw_color(Color::GREEN);

    // let rect = Rect::new(100, 150, 200, 50);
    // buf.draw_rect(&rect);

    buf.draw(map);

    save_ppm("images/map.ppm", &buf).expect("Could not write to file.");
}
