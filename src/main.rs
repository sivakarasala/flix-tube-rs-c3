use std::{
    fs::{self, File},
    io::Read,
    net::SocketAddr,
};

use axum::{
    body::{self, Full},
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/video", get(handle_static_video));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_static_video() -> impl IntoResponse {
    let path = "./videos/SampleVideo_1280x720_1mb.mp4";
    let mime_type = mime_guess::from_path(path.clone()).first_or_text_plain();
    let mut file_contents = Vec::new();
    let mut file = File::open(&path).expect("Unable to open video file");
    file.read_to_end(&mut file_contents)
        .expect("Unable to read video file");
    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type.as_ref()).unwrap(),
        )
        .body(body::boxed(Full::from(file_contents)))
        .unwrap()
}
