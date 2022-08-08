use crate::application::product::ProductServiceInterface;
use actix_web::{
    dev::Server,
    middleware,
    web::{self},
    App, HttpServer,
};
use error_stack::{Context, Report};
use std::{self, time::Duration};

use super::handlers::product_handler::get_product;

#[derive(Debug)]
pub struct WebServerError(pub String);

impl Context for WebServerError {}

impl std::fmt::Display for WebServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("web server error")
    }
}

pub struct WebServer {
    service: Box<dyn ProductServiceInterface>,
}

impl WebServer {
    pub fn new(service: Box<dyn ProductServiceInterface>) -> Self {
        WebServer { service }
    }

    pub async fn serve(&self) -> error_stack::Result<Server, WebServerError> {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        let http_server = HttpServer::new(|| {
            App::new()
                .wrap(middleware::Logger::default())
                .service(web::resource("/product/{id}").to(get_product))
        })
        .client_request_timeout(Duration::from_secs(10))
        .shutdown_timeout(30)
        .client_disconnect_timeout(Duration::from_secs(5));
        //
        match http_server.bind(("127.0.0.1", 9001)) {
            Ok(server) => Ok(server.run()),
            Err(_) => Err(Report::new(WebServerError("bind addr error".to_string()))),
        }
    }
}
