
use axum::{
    routing::get,
    Router,
};

mod zircon_proxy;

// async fn proxy(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     let uri = format!("http://remote-server.com{}", req.uri().path_and_query().unwrap().to_string());
//     let mut proxied_req = req;
//     *proxied_req.uri_mut() = uri.parse().unwrap();
//     let client = Client::new();
//     client.request(proxied_req).await
// }

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3006").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
