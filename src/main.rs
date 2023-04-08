use warp::Filter;

#[tokio::main]
async fn main() {
    let ip = warp::filters::path::end()
        .and(warp::addr::remote())
        .map(|addr: Option<std::net::SocketAddr>| format!("{}", addr.unwrap()));

    warp::serve(ip).run(([0, 0, 0, 0], 8080)).await;
}
