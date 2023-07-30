// Import necessary actix_web components.
use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use std::net::TcpListener;

// The health_check route handler
async fn health_check(_req: HttpRequest) -> HttpResponse {
    // Always return a 200 OK response
    HttpResponse::Ok().finish()
}

//  fluent API (i.e. chaining method calls one after the other)
// Setup and run the server
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}
