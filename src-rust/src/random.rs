use rand::{thread_rng, Rng};
use crate::game::Pos;

fn random_range(min: usize, max: usize) -> usize {
    thread_rng().gen_range(min..max)
}

pub fn random_fields(width: usize, height: usize, num_fields: usize) -> impl Iterator<Item = Pos> {
    let mut positions = vec![];

    let mut i = 0;
    while i < num_fields {
        let temp = (random_range(0, width), random_range(0, height));
        if positions.contains(&temp) { continue; }
        positions.push(temp);
        i += 1;
    }

    positions.into_iter()
}
