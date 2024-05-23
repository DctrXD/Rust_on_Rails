mod controllers;
mod models;
mod storage;

use warp::Filter;

#[tokio::main]
async fn main() {
    // Configuração do banco de dados (se necessário)

    let routes = warp::path("users")
        .and(controllers::users::routes()) 
        .with(warp::cors().allow_any_origin()); 

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}