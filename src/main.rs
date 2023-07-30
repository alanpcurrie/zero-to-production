use std::net::TcpListener;
use zero2prod::run;

// The entry point of the application
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create a TCP listener on localhost and on port 8000
    let address = TcpListener::bind("127.0.0.1:8000")?;
    // Run the server and await its completion
    run(address)?.await
}
