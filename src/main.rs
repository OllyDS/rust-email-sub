use ::rust_email_sub::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind url & port.");

    run(listener)?.await
}
