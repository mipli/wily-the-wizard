extern crate rand;
extern crate geo;
extern crate map_generator;

use rand::{XorShiftRng, SeedableRng};

fn main() {
    let mut rng: XorShiftRng = SeedableRng::from_seed([0, 1, 3, 4]);
    let map = map_generator::roomsy::generate(40, 20, 4, &mut rng);
    println!("{:?}", map);
}
