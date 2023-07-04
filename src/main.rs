use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};


#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(handler));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();


}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

