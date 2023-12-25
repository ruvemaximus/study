mod map;

use map::Map;

fn main() {
    let mut m: Map<i8> = Map::new();
    m["2".to_string()] = 2;
    m["1".to_string()] = 1;

    dbg!(m);
}
