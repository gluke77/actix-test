use reqwest;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_tracing::TracingMiddleware;

pub fn child_port() -> u16 {
    let default_port = 8080;
    std::env::var("CHILD_PORT").map_or(default_port, |v| v.parse::<u16>().unwrap_or(default_port))
}

pub fn client() -> ClientWithMiddleware {
    reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(TracingMiddleware::default())
        .build()
}
