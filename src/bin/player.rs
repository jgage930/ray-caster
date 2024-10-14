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
        1   11   1
        1        1
        1        1
        1111111111
    "
    .trim();

    let map = Map::new(64, map_file);
    let player = Player::new(100, 100);

    let rays = player.cast_rays(&map);
    let mut buf = FrameBuffer::new(512, 512, &Color::WHITE);

    buf.set_draw_color(Color::RED);
    buf.render_3d(&rays);

    save_ppm("images/3d_player.ppm", &buf).expect("Could not write to file.");
}
