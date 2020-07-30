use warp::{Filter, Rejection, Reply};

pub fn health() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health").and(warp::get()).map(handle)
}

fn handle() -> String {
    "Oh, hi Mark!".into()
}
