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

pub fn simulated_annealing<S, T>(state: &S, max_k: usize, temperature: T) -> S
where
    S: State + Clone,
    T: Fn(usize) -> f64,
{
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(0.0, 1.0);

    let mut current_state = state.clone();

    for k in 0..max_k {
        let t = temperature(k);

        let next_state = current_state.get_next_states().find(|candidate| {
            let acceptability = current_state.acceptability(candidate, t);
            let sampling = uniform.sample(&mut rng);

            acceptability >= sampling
        });

        if let Some(next_state) = next_state {
            current_state = next_state;
        } else {
            break;
        }
    }

    current_state
}
