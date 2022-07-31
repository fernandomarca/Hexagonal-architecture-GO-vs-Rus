use adapters::db::product_db::ProductDb;
use application::product_service::ProductService;
use rusqlite::Connection;

mod adapters;
mod application;
fn main() {
    let db = Connection::open("sqlite.db").unwrap();
    let product_db_adapter = ProductDb::new(db);
    let product_service = ProductService::new(Box::new(product_db_adapter));

    let product = product_service.create("Product Exemplo", 31.0).unwrap();
    let product = product_service.enable(product.as_ref());
    // println!("{}", product.unwrap());
}
