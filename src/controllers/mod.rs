use warp::Filter;

pub async fn hello_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Hello, world!"))
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(hello_handler)
}
