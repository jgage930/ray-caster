use ray_caster::*;

/// Hello world for the ray caster.  
/// Save a ppm image to disk of a solid background color.
fn main() {
    let buf = FrameBuffer::new(500, 500, &Color::RED);
    save_ppm("images/hello.ppm", &buf).expect("Could not write file.");
}
