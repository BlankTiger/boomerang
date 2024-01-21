use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    response::Html,
    routing, Router,
};
use boomeranglib::make_boomerang;
use std::collections::HashMap;
use std::io::Write;

async fn create_boomerang(Query(params): Query<HashMap<String, String>>, mut multipart: Multipart) {
    let filename = "current_file.mp4";
    let mut file = std::fs::File::create(filename).unwrap();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
    let zero = "0".to_string();
    let one = "1".to_string();
    let from_sec = params.get("from_sec").unwrap_or(&zero);
    let to_sec = params.get("to_sec").unwrap_or(&zero);
    let speed = params.get("speed").unwrap_or(&one).parse::<f64>().unwrap();
    make_boomerang(filename, from_sec, to_sec, Some(1), Some(speed)).unwrap();
    println!(
        "from_sec: {}, to_sec: {}, speed: {}",
        from_sec, to_sec, speed
    );
}

async fn website() -> Html<String> {
    let html = include_str!("../website/index.html");
    let ip = get_local_ip::network().expect("Network to work").ip;
    let html = html.replace("{ip}", &ip);
    Html(html)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(website))
        .route("/make_boomerang", routing::post(create_boomerang))
        .layer(DefaultBodyLimit::max(100000000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
