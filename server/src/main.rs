use axum::routing::get_service;
use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::routing::SpaRouter;
use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "5001")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    let spa = SpaRouter::new("/assets/", "../dist");

    let app = Router::new()
        .route("/api/hello", get(hello))
        .merge(spa)
        // .fallback(get_service(ServeDir::new("../dist")).handle_error(handle_error))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{}", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn hello() -> impl IntoResponse {
    "hello from server!"
}

async fn handle_error(_: std::io::Error) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, "Something went wrong...")
}
