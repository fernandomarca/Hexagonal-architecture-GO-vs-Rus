use actix_web::{http::header::ContentType, web, HttpResponse, Responder};

use crate::{
    application::product::{Product, ProductServiceInterface},
    cmd::connection_db_factory,
};

pub async fn get_product(id: web::Path<String>) -> impl Responder {
    let service = connection_db_factory();
    let product = service.get(id.to_string());

    match product {
        Ok(p) => {
            let product_serialized = Product {
                id: p.get_id().to_owned(),
                name: p.get_name().to_owned(),
                price: p.get_price(),
                status: p.get_status(),
            };
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(product_serialized)
        }
        Err(_e) => HttpResponse::NotFound().finish(),
    }
}
