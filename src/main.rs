#![allow(unused)]

use std::net::SocketAddr;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    //created the route
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    // region:    ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    //serve the router  using server and also intomakeservice
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: --Start server

    // region:   --- Handler Hello
    async fn handler_hello() -> impl IntoResponse{
        println!("->> {:<12} - handler_hello", "HANDLER");

        Html("Hello <strong>World!!!</strong>")
    }
    // ENDREGION:  --- Handler Hello
}
