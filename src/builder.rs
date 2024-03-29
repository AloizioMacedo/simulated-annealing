use crate::simulated_annealing::SimulatedAnnealing;

pub struct SimulatedAnnealingBuilder {
    temperature: Box<dyn Fn(usize) -> f64>,
    max_k: usize,
}

impl Default for SimulatedAnnealingBuilder {
    fn default() -> Self {
        SimulatedAnnealingBuilder {
            temperature: Box::new(|k| 1.0 - 0.01 * k as f64),
            max_k: 100,
        }
    }
}

impl SimulatedAnnealingBuilder {
    pub fn new() -> SimulatedAnnealingBuilder {
        SimulatedAnnealingBuilder::default()
    }

    pub fn with_temperature_and_max_iter<T>(mut self, temperature: T, max_k: usize) -> Self
    where
        T: Fn(usize) -> f64 + 'static,
    {
        self.temperature = Box::new(temperature);
        self.max_k = max_k;

        self
    }

    pub fn build(self) -> SimulatedAnnealing {
        SimulatedAnnealing {
            temperature: self.temperature,
            max_k: self.max_k,
        }
    }
}
