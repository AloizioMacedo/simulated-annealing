use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

use crate::simulated_annealing::State;

#[derive(Debug, Clone)]
struct Point(f64, f64);

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

#[derive(Clone, Debug)]
struct Tsp {
    state: Vec<Point>,
}

impl Tsp {
    fn energy(&self) -> f64 {
        self.state
            .iter()
            .cycle()
            .skip(1)
            .take(self.state.len())
            .zip(&self.state)
            .map(|(next, previous)| next.distance(previous))
            .sum()
    }
}

impl State for Tsp {
    fn acceptability(&self, new: &Self, t: f64) -> f64 {
        let new_energy = Tsp::energy(new);
        let my_energy = self.energy();

        if new_energy < my_energy {
            1.0
        } else {
            f64::exp(-(new_energy - my_energy) / t)
        }
    }

    fn get_next_states(&self) -> impl Iterator<Item = Self> + '_ {
        let mut rng = thread_rng();
        let n = self.state.len();

        let mut swaps: Vec<_> = (0..n).tuple_combinations::<(usize, usize)>().collect();

        swaps.shuffle(&mut rng);

        swaps.into_iter().map(|(i, j)| {
            let mut state = self.state.clone();

            state.swap(i, j);

            Tsp { state }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulated_annealing::simulated_annealing;

    #[test]
    fn test_distance() {
        assert_eq!(Point(0.0, 0.0).distance(&Point(3.0, 4.0)), 5.0)
    }

    #[test]
    fn test_energy() {
        let tsp = Tsp {
            state: vec![Point(0.0, 0.0), Point(3.0, 0.0), Point(3.0, 4.0)],
        };
        assert_eq!(tsp.energy(), 12.0);
    }

    #[test]
    fn test_tsp() {
        let state = vec![
            Point(0.0, 0.0),
            Point(0.0, 2.0),
            Point(2.0, 0.0),
            Point(2.0, 2.0),
            Point(1.0, 3.0),
            Point(1.0, -1.0),
        ];

        let tsp = Tsp { state };

        let final_state = simulated_annealing(&tsp, 100, |k| 1.0 - (0.01 * k as f64));

        let correct_result = Tsp {
            state: vec![
                Point(0.0, 0.0),
                Point(0.0, 2.0),
                Point(1.0, 3.0),
                Point(2.0, 2.0),
                Point(2.0, 0.0),
                Point(1.0, -1.0),
            ],
        };

        let error = (final_state.energy() - correct_result.energy()).abs();

        if error > 0.0001 {
            panic!(
                "error of {error} from final_state energy '{}' to '{}'. States are: {:?}",
                final_state.energy(),
                correct_result.energy(),
                final_state.state,
            )
        }
    }

    #[test]
    fn test_big_polygon() {
        let n_vertices = 20;

        let z =
            num::complex::Complex::from_polar(1.0, 2.0 * std::f64::consts::PI / n_vertices as f64);

        let state: Vec<_> = (0..n_vertices)
            .map(|i| z.powi(i))
            .map(|z| Point(z.re, z.im))
            .collect();

        let mut tsp = Tsp { state };

        let best_energy = tsp.energy();

        let mut rng = thread_rng();
        tsp.state.shuffle(&mut rng);

        let energy_after_shuffle = tsp.energy();
        println!("energy after shuffling: {energy_after_shuffle}");

        let final_state = simulated_annealing(&tsp, 10_000, |k| 1.0 - (0.0001 * k as f64));

        let error = (best_energy - final_state.energy()).abs();

        if error > 0.0001 {
            panic!(
                "error of {error} from final_state energy '{}' to '{}'. States are: {:?}",
                final_state.energy(),
                best_energy,
                final_state.state,
            )
        }
    }
}
