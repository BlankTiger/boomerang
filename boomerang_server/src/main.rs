use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    response::Html,
    routing, Router,
};
use boomeranglib::make_boomerang;
use color_eyre::Report;
use tracing::info;
use std::collections::HashMap;
use std::io::Write;
use tracing_subscriber::EnvFilter;

async fn create_boomerang(Query(params): Query<HashMap<String, String>>, mut multipart: Multipart) {
    let curr_file = "current_file.mp4".to_string();
    let filename = params.get("filename").unwrap_or(&curr_file);
    let mut file = std::fs::File::create(filename).unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).unwrap();

        info!("Length of `{}` is {} bytes", name, data.len());
    }
    let zero = "0".to_string();
    let one = "1".to_string();
    println!("{:?}", params);
    let from_sec = params.get("from_sec").unwrap_or(&zero);
    let to_sec = params.get("to_sec").unwrap_or(&zero);
    let speed = params.get("speed").unwrap_or(&one).parse::<f64>().unwrap();
    make_boomerang(filename, from_sec, to_sec, Some(1), Some(speed)).unwrap();
    info!(
        "from_sec: {}, to_sec: {}, speed: {}",
        from_sec, to_sec, speed
    );
}

async fn website() -> Html<String> {
    let html = include_str!("../website/index.html");
    let ip = local_ip_address::local_ip()
        .expect("Network to work")
        .to_string();
    info!("IP: {}", ip);
    let html = html.replace("{ip}", &ip);
    Html(html)
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    setup()?;
    let app = Router::new()
        .route("/", routing::get(website))
        .route("/make_boomerang", routing::post(create_boomerang))
        .layer(DefaultBodyLimit::max(100000000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

pub fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
