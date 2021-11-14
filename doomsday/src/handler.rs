pub async fn hello_world(_req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
    Ok(hyper::Response::new("Hello, World".into()))
}