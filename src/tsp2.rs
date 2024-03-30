use itertools::Itertools;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(Serialize, Deserialize)]
pub struct Tsp {
    pub points: Vec<Point>,
}

impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

pub fn energy(points: &[Point]) -> f64 {
    points
        .iter()
        .cycle()
        .skip(1)
        .take(points.len())
        .zip(points)
        .map(|(next, previous)| next.distance(previous))
        .sum()
}

pub fn acceptability(me: &[Point], new: &[Point], t: f64) -> f64 {
    let new_energy = energy(new);
    let my_energy = energy(me);

    if new_energy < my_energy {
        1.0
    } else {
        f64::exp(-(new_energy - my_energy) / t)
    }
}

pub fn simulated_annealing(state: &[Point], max_k: usize) -> Vec<Point> {
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(0.0, 1.0);

    let current_state = &mut state.to_vec();
    let n = current_state.len();

    let mut holder = vec![Point::default(); n];

    let mut swaps: Vec<_> = (0..n).tuple_combinations::<(usize, usize)>().collect();

    'outer: for k in 0..max_k {
        let t = 1.0 / k as f64;

        swaps.shuffle(&mut rng);

        for (i, j) in swaps.iter() {
            holder.copy_from_slice(current_state);
            holder.swap(*i, *j);

            if acceptability(current_state, &holder, t) >= uniform.sample(&mut rng) {
                current_state.copy_from_slice(&holder);
                continue 'outer;
            }
        }

        break;
    }

    current_state.to_vec()
}

#[cfg(test)]
mod tests {

    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(Point(0.0, 0.0).distance(&Point(3.0, 4.0)), 5.0)
    }

    #[test]
    fn test_tsp() {
        let state = &[
            Point(0.0, 0.0),
            Point(0.0, 2.0),
            Point(2.0, 0.0),
            Point(2.0, 2.0),
            Point(1.0, 3.0),
            Point(1.0, -1.0),
        ];

        let correct_result = &[
            Point(0.0, 0.0),
            Point(0.0, 2.0),
            Point(1.0, 3.0),
            Point(2.0, 2.0),
            Point(2.0, 0.0),
            Point(1.0, -1.0),
        ];

        let final_state = simulated_annealing(state, 1000);

        let error = (energy(&final_state) - energy(correct_result)).abs();

        if error > 0.001 {
            panic!("error: {error}")
        }
    }

    #[test]
    fn test_big_polygon2() {
        let n_vertices = 20;

        let z =
            num::complex::Complex::from_polar(1.0, 2.0 * std::f64::consts::PI / n_vertices as f64);

        let mut state: Vec<_> = (0..n_vertices)
            .map(|i| z.powi(i))
            .map(|z| Point(z.re, z.im))
            .collect();

        let mut rng = thread_rng();
        state.shuffle(&mut rng);

        let result = simulated_annealing(&state, 3000);

        println!("{}", energy(&result));
    }
}
