use warp::{Reply, Rejection, reply::Json}; 

pub async fn index() -> Result<impl Reply, Rejection> {
    let users = vec![
        User { id: 1, name: "Usuário 1".to_string(), email: "usuario1@example.com".to_string() },
        User { id: 2, name: "Usuário 2".to_string(), email: "usuario2@example.com".to_string() },
    ];

    Ok(warp::reply::json(&users))
}

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(index) 
        // ... outras rotas (ex: create, show, update, delete)
}