use ray_caster::engine::map::Map;

fn main() {
    let map = Map::new("maps/test_map.txt").expect("Could not create map.");
}
