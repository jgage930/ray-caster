use ray_caster::*;

fn main() {
    let mut buf = FrameBuffer::new(500, 500, &Color::BLACK);
    buf.set_draw_color(Color::GREEN);

    let rect = Rect::new(100, 150, 200, 50);
    buf.draw_rect(&rect);

    save_ppm("images/map.ppm", &buf).expect("Could not write to file.");
}
