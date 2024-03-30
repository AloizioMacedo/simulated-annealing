use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use itertools::Itertools;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    SeedableRng,
};
use serde_json::json;
use simulated_annealing::tsp2::{acceptability, Point, Tsp};

fn generate_circle(n_vertices: usize) -> Vec<Point> {
    let z = num::Complex::from_polar(1.0, 2.0 * std::f64::consts::PI / n_vertices as f64);

    (0..(n_vertices as i32))
        .map(|i| z.powi(i))
        .map(|z| Point(z.re, z.im))
        .collect()
}

fn generate_random(n_vertices: usize) -> Vec<Point> {
    let mut rng = rand::rngs::StdRng::from_entropy();

    let uniform = Uniform::new_inclusive(0.0, 1.0);

    (0..n_vertices)
        .map(|_| Point(uniform.sample(&mut rng), uniform.sample(&mut rng)))
        .collect()
}

fn geometric_swap<T>(points: &mut [T], i: usize, j: usize) {
    let (i, j) = (i.min(j), i.max(j));

    points.swap(i, j);

    points[(i + 1)..j].reverse();
}

fn greedy(mut points: Vec<Point>) -> Vec<Point> {
    let mut new_order = Vec::new();

    let Some(first) = points.pop() else {
        return new_order;
    };

    let mut last_to_be_added = first;
    new_order.push(first);

    while !points.is_empty() {
        let (i, point) = points
            .iter()
            .enumerate()
            .min_by(|(_, p1), (_, p2)| {
                p1.distance(&last_to_be_added)
                    .partial_cmp(&p2.distance(&last_to_be_added))
                    .unwrap()
            })
            .unwrap();

        new_order.push(*point);
        last_to_be_added = *point;
        points.remove(i);
    }

    new_order
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let n_vertices = 30;

    let mut state = generate_random(n_vertices);

    let mut rng = rand::rngs::StdRng::from_entropy();

    state = greedy(state);

    let uniform = Uniform::new_inclusive(0.0, 1.0);

    let current_state = &mut state.to_vec();
    let n = current_state.len();

    let mut holder = vec![Point::default(); n];
    let mut tuple_combs = (1..(n - 1))
        .tuple_combinations::<(usize, usize)>()
        .collect_vec();

    'outer: for k in 0..3000 {
        let t = 1.0 / k as f64;

        tuple_combs.shuffle(&mut rng);

        for (i, j) in tuple_combs.iter() {
            holder.copy_from_slice(current_state);
            geometric_swap(&mut holder, *i, *j);

            if acceptability(current_state, &holder, t) >= uniform.sample(&mut rng) {
                current_state.copy_from_slice(&holder);
                eprintln!("Iteration: {}. Swapped: {}, {}", k, i, j);

                let jsonified = json!(Tsp {
                    points: current_state.to_vec(),
                });

                socket
                    .send(axum::extract::ws::Message::Text(jsonified.to_string()))
                    .await
                    .unwrap();
                continue 'outer;
            }
        }

        break;
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/ws", get(handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use simulated_annealing::tsp2::energy;

    use super::*;

    #[test]
    fn test_geometric_swap() {
        let mut points = vec![0, 1, 2, 3, 4, 0, 1, 2, 10, 11, 12, 13, 14];

        geometric_swap(&mut points, 5, 8);

        assert_eq!(points, vec![0, 1, 2, 3, 4, 10, 2, 1, 0, 11, 12, 13, 14]);

        let mut points = vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4, 5, 6, 10, 11, 12];

        geometric_swap(&mut points, 5, 12);

        assert_eq!(points, vec![0, 1, 2, 3, 4, 10, 6, 5, 4, 3, 2, 1, 0, 11, 12]);
    }

    #[test]
    fn test_greedy() {
        let correct_order = vec![
            Point(0.0, 0.0),
            Point(0.0, 1.0),
            Point(0.0, 2.0),
            Point(1.0, 2.0),
            Point(2.0, 2.0),
            Point(2.0, 1.0),
            Point(2.0, 0.0),
            Point(1.0, 0.0),
        ];

        let unordered = vec![
            Point(0.0, 0.0),
            Point(2.0, 1.0),
            Point(0.0, 2.0),
            Point(1.0, 2.0),
            Point(2.0, 0.0),
            Point(0.0, 1.0),
            Point(1.0, 0.0),
            Point(2.0, 2.0),
        ];

        let ordered = greedy(unordered);

        assert_eq!(energy(&ordered), energy(&correct_order));
    }
}
