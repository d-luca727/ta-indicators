use crate::configuration::get_configuration;
use crate::crypto_client::CryptoClient;
use crate::routes::fibonacci_retracement::fibonacci_extension;
use crate::routes::{
    aroon_oscillator, exponential_moving_average, fibonacci_retracement, health_check, rsi,
    simple_moving_average, stochastic_oscillator,
};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
                "/simple_moving_average",
                web::get().to(simple_moving_average),
            )
            .route(
                "/fibonacci_retracement",
                web::get().to(fibonacci_retracement),
            )
            .route("/fibonacci_extension", web::get().to(fibonacci_extension))
            .route("/rsi", web::get().to(rsi))
            .route("/aroon_oscillator", web::get().to(aroon_oscillator))
            .route(
                "/stochastic_oscillator",
                web::get().to(stochastic_oscillator),
            )
            .route(
                "/exponential_moving_average",
                web::get().to(exponential_moving_average),
            )
            .app_data(crypto_client.to_owned())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
