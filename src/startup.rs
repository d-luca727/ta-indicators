use crate::configuration::get_configuration;
use crate::crypto_client::CryptoClient;
use crate::routes::fibonacci_retracement::fibonacci_extension;
use crate::routes::{
    aroon_oscillator, exponential_moving_average, fibonacci_retracement, health_check, rsi,
    simple_moving_average, stochastic_oscillator,
};
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpServer};

use std::net::TcpListener;
use std::path::PathBuf;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let crypto_client = Data::new(CryptoClient::new(
        configuration.crypto_client.base_url,
        configuration.crypto_client.auth_token,
    ));
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route(
                "/simple_moving_average/{coin}/{time}",
                web::get().to(simple_moving_average),
            )
            .route(
                "/fibonacci_retracement/{coin}/{market}",
                web::get().to(fibonacci_retracement),
            )
            .route(
                "/fibonacci_extension/{coin}/{market}",
                web::get().to(fibonacci_extension),
            )
            .route("/rsi/{coin}", web::get().to(rsi))
            .route("/aroon_oscillator/{coin}", web::get().to(aroon_oscillator))
            .route(
                "/stochastic_oscillator/{coin}",
                web::get().to(stochastic_oscillator),
            )
            .route(
                "/exponential_moving_average/{coin}",
                web::get().to(exponential_moving_average),
            )
            /* .route("/docs", web::get().to(docs))
            .route("/json", web::get().to(json_get)) */
            .app_data(crypto_client.to_owned())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

/* async fn docs(_req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let html_path = base_path.join("configuration/static/index.html");
    Ok(NamedFile::open(html_path)?)
}

async fn json_get(_req: HttpRequest) -> Result<NamedFile, std::io::Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let json_path = base_path.join("configuration/static/openapi.json");
    Ok(NamedFile::open(json_path)?)
}
 */
