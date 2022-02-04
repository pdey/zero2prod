use zero2prod::configuration::get_config;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load configuration");

    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = std::net::TcpListener::bind(address)?;
    run(listener).await?.await
}
