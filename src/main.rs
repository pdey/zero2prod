use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind an address");
    let port = listener.local_addr().unwrap().port();
    println!("Application running on http://127.0.0.1:{}", port);
    run(listener).await?.await
}
