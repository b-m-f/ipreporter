extern crate pretty_env_logger;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let ip = warp::filters::path::end()
        .and(warp::addr::remote())
        .map(|addr: Option<std::net::SocketAddr>| format!("{}", addr.unwrap()));

    let cert_path: Option<String> = match std::env::var("CERT_PATH") {
        Ok(path) => Some(path),
        _ => None,
    };
    let key_path: Option<String> = match std::env::var("KEY_PATH") {
        Ok(path) => Some(path),
        _ => None,
    };
    let http_port: u16 = match std::env::var("HTTP_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        _ => 80,
    };
    let https_port: u16 = match std::env::var("HTTPS_PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        _ => 443
    };
    if cert_path.and(key_path).is_some() {
        warp::serve(ip)
            .tls()
            .cert_path(std::env::var("CERT_PATH").unwrap())
            .key_path(std::env::var("KEY_PATH").unwrap())
            .run(([0, 0, 0, 0], https_port))
            .await;
    } else {
        warp::serve(ip).run(([0, 0, 0, 0], http_port)).await;
    }
}
