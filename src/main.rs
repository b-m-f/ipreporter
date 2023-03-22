use tide::log;
use tide::utils::After;
use tide::Request;
use tide::Response;
use tide::StatusCode;
use tide_rustls::TlsListener;

use ctrlc;

async fn caller_ip(req: Request<()>) -> tide::Result {
    match req.remote() {
        Some(ip) => {
            let peer_host: String = String::from(ip).split(":").take(1).collect();
            Ok(Response::builder(StatusCode::Ok)
                .body(format!("{}", peer_host))
                .build())
        }
        None => Ok(Response::new(StatusCode::BadRequest)),
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // Make sure we can exit

    ctrlc::set_handler(move || panic!("Server terminated")).expect("Error setting Ctrl-C handler");

    log::start();
    let mut app = tide::new();
    app.with(After(|response: Response| async move {
        let response = match response.status() {
            StatusCode::BadRequest => Response::builder(400).build(),

            _ => response,
        };

        Ok(response)
    }));
    app.at("/").get(caller_ip);
    //
    let cert_path: Option<String> = match std::env::var("CERT_PATH") {
        Ok(path) => Some(path),
        _ => None,
    };
    let key_path: Option<String> = match std::env::var("KEY_PATH") {
        Ok(path) => Some(path),
        _ => None,
    };

    if cert_path.and(key_path).is_some() {
        log::info!("Starting server on Port 443 in HTTPS mode");
        app.listen(
            TlsListener::build()
                .addrs("0.0.0.0:443")
                .cert(std::env::var("CERT_PATH").unwrap())
                .key(std::env::var("KEY_PATH").unwrap()),
        )
        .await?;
    } else {
        log::info!("Starting server on Port 80 in HTTP mode");
        app.listen("0.0.0.0:80").await?;
    }
    Ok(())
}
