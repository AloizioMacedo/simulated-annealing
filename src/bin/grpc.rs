use tonic::{transport::Server, Request, Response, Status};

use grpc_sim_ann::data_points_provider_server::{DataPointsProvider, DataPointsProviderServer};
use grpc_sim_ann::{DataPoint, DataPoints, Empty};

pub mod grpc_sim_ann {
    tonic::include_proto!("grpc_sim_ann"); // The string specified here must match the proto package name
}

use itertools::Itertools;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    SeedableRng,
};
use simulated_annealing::tsp2::{acceptability, Point, Tsp};

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

fn get_iterations() -> Vec<Tsp> {
    let n_vertices: usize = std::env::var("N_VERTICES")
        .map(|x| {
            x.parse()
                .expect("The env variable 'N_VERTICES' should be convertible to integer.")
        })
        .unwrap_or(40);

    let mut state = generate_random(n_vertices);

    let mut rng = rand::rngs::StdRng::from_entropy();

    state = greedy(state);

    let uniform = Uniform::new_inclusive(0.0, 1.0);

    let current_state = &mut state.to_vec();
    let n = current_state.len();

    let mut holder = vec![Point::default(); n];
    let mut tuple_combs = (1..n).tuple_combinations::<(usize, usize)>().collect_vec();
    tuple_combs.retain(|tup| *tup != (1, n - 1));

    let mut results = Vec::new();

    'outer: for k in 0..5000 {
        let t = 10.0 / (k as f64).powf(1.2);

        tuple_combs.shuffle(&mut rng);

        for (i, j) in tuple_combs.iter() {
            holder.copy_from_slice(current_state);
            geometric_swap(&mut holder, *i, *j);

            if acceptability(current_state, &holder, t) >= uniform.sample(&mut rng) {
                current_state.copy_from_slice(&holder);

                let (x, y) = current_state.iter().map(|p| (p.0, p.1)).unzip();

                results.push(Tsp { x, y });
                continue 'outer;
            }
        }

        break;
    }

    results
}

#[derive(Debug, Default)]
pub struct MyProvider {}

#[tonic::async_trait]
impl DataPointsProvider for MyProvider {
    async fn get_data_points(
        &self,
        _: Request<Empty>, // Accept request of type HelloRequest
    ) -> Result<Response<DataPoints>, Status> {
        let iterations = get_iterations();

        let data = iterations
            .into_iter()
            .map(|tsp| DataPoint { x: tsp.x, y: tsp.y })
            .collect_vec();

        let data_points = DataPoints { data };

        Ok(Response::new(data_points)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyProvider::default();

    Server::builder()
        .add_service(DataPointsProviderServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
