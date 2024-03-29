use crate::builder::SimulatedAnnealingBuilder;
use std::ops::Deref;

use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

pub trait State {
    fn acceptability(&self, new: &Self, t: f64) -> f64;

    fn get_next_states(&self) -> impl Iterator<Item = Self>
    where
        Self: Clone;
}

pub struct SimulatedAnnealing {
    pub(crate) temperature: Box<dyn Fn(usize) -> f64>,
    pub(crate) max_k: usize,
}

impl Default for SimulatedAnnealing {
    fn default() -> Self {
        Self {
            temperature: Box::new(|k| 1.0 - 0.01 * k as f64),
            max_k: 100,
        }
    }
}

impl SimulatedAnnealing {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> SimulatedAnnealingBuilder {
        SimulatedAnnealingBuilder::default()
    }

    pub fn run<S>(&self, state: &S) -> S
    where
        S: State + Clone,
    {
        let mut rng = thread_rng();
        let uniform = Uniform::new_inclusive(0.0, 1.0);

        let mut current_state = state.clone();

        let temperature = self.temperature.deref();

        for k in 0..self.max_k {
            let t = temperature(k);

            let next_state = current_state.get_next_states().find(|candidate| {
                current_state.acceptability(candidate, t) >= uniform.sample(&mut rng)
            });

            if let Some(next_state) = next_state {
                current_state = next_state;
            } else {
                break;
            }
        }

        current_state
    }
}
