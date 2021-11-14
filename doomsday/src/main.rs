#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = hyper::service::make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, std::convert::Infallible>(hyper::service::service_fn(doomsday::hello_world))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    println!("listening on http://127.0.0.1:3000");
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}