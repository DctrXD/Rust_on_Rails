mod controllers;
mod models;
mod storage;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Initialize routes
    let routes = controllers::routes();

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
