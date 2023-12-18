mod config;
mod trace;

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{extract::State, routing::post};
use axum_client_ip::InsecureClientIp;
use clap::{arg, command, value_parser};
use ipinfo::{BatchReqOpts, IpInfo, IpInfoConfig};
use tokio::sync::Mutex;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Clap path to config file
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -p --port <PORT> "Sets the port to listen on"
            )
            .default_value("3000")
            .value_parser(value_parser!(u16)),
        )
        .get_matches();

    let config_path = matches
        .get_one::<PathBuf>("config")
        .expect("config is required");

    let port: u16 = matches.get_one("port").cloned().unwrap();

    let config = config::Config::load(config_path.to_str().expect("config path is not valid"))
        .expect("failed to parse config");

    let geo_ip = Arc::new(Mutex::new(
        IpInfo::new(IpInfoConfig {
            token: Some(config.token),
            ..Default::default()
        })
        .unwrap(),
    ));

    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");
    info!("Serving static files from {:?}", static_dir);

    let static_files_service = ServeDir::new(static_dir).append_index_html_on_directories(true);

    // Axum api
    let app = axum::Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/trace", post(trace))
        .nest_service("/", static_files_service)
        .with_state(geo_ip);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

// Traceroute trace to location api
//
// POST /trace
async fn trace(
    State(state): State<Arc<Mutex<IpInfo>>>,
    insecure_ip: InsecureClientIp,
    trace: String,
) -> String {
    // The first ip address is the ip of the client (as a string)
    let client_ip = insecure_ip.0.to_string();

    // Parse the trace into a vector of ip addresses
    let mut ips = trace::parse_trace(&trace);
    ips.insert(0, client_ip.as_str());

    // Lookup the ip addresses
    let details = state
        .lock()
        .await
        .lookup_batch(&ips, BatchReqOpts::default())
        .await
        .unwrap();

    // Sort the results by hop
    ips.iter()
        .enumerate()
        .flat_map(|(i, &ip)| details.get(ip).map(|details| (i, details)))
        .map(|(i, details)| (i, details.loc.clone()))
        .filter(|(_, loc)| !loc.is_empty())
        .fold(String::new(), |mut acc, (i, loc)| {
            acc.push_str(&format!("{},{}\n", i, loc));
            acc
        })
}
