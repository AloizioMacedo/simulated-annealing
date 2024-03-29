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

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let n_vertices = 30;

    let mut state = generate_random(n_vertices);

    let mut rng = rand::rngs::StdRng::from_entropy();

    state.shuffle(&mut rng);

    let uniform = Uniform::new_inclusive(0.0, 1.0);

    let current_state = &mut state.to_vec();
    let n = current_state.len();

    let mut holder = vec![Point::default(); n];
    let mut tuple_combs = (0..n).tuple_combinations::<(usize, usize)>().collect_vec();

    'outer: for k in 0..5000 {
        let t = 20.0 / k as f64;

        tuple_combs.shuffle(&mut rng);

        for (i, j) in tuple_combs.iter() {
            holder.copy_from_slice(current_state);
            holder.swap(*i, *j);

            if acceptability(current_state, &holder, t) >= uniform.sample(&mut rng) {
                current_state.copy_from_slice(&holder);

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
