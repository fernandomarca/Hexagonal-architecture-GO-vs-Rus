use error_stack::{Report, Result};
use rusqlite::Connection;

use crate::application::product::{Product, ProductInterface, ProductPersistanceInterface};

use super::product_db::{DbError, ProductDb};

fn setup() -> Result<Connection, DbError> {
    let db = connection_db_factory()?;
    create_table_products(&db)?;
    create_product(&db)?;
    Ok(db)
}
fn connection_db_factory() -> Result<Connection, DbError> {
    let db = Connection::open_in_memory();
    db.or_else(|_e| {
        return Err(Report::new(DbError).attach_printable(format!("open db connection error")));
    })
}

fn create_table_products(db: &Connection) -> Result<usize, DbError> {
    let table = "CREATE TABLE PRODUCTS (
    id    STRING,
    name  STRING,
    price FLOAT,
    status STRING
)";
    let result = db.execute(
        table,
        (), // empty list of parameters.
    );
    result.or_else(|_e| {
        return Err(
            Report::new(DbError).attach_printable(format!("{table} sql cannot be converted"))
        );
    })
}

fn create_product(db: &Connection) -> Result<usize, DbError> {
    let insert = "insert into products (
      id,
      name,
      price,
      status
    ) values (?1, ?2, ?3, ?4)";
    let result = db.execute(insert, ("abc", "Product Test", 0, "disabled"));
    result.or_else(|_e| {
        return Err(
            Report::new(DbError).attach_printable(format!("{},sql cannot be converted", insert))
        );
    })
}

#[test]
fn test_product_db_get() {
    use pretty_assertions::assert_eq;
    let db = setup().expect("open db connection error");
    let product_db = ProductDb::new(db);
    let product = product_db.get("abc").unwrap();

    assert_eq!(product.get_name(), "Product Test");
    assert_eq!(product.get_price(), 0.0);
    assert_eq!(product.get_status(), "disabled");
}

#[test]
fn test_product_db_save() {
    use pretty_assertions::assert_eq;
    let db = setup().expect("open db connection error");
    let product_db = ProductDb::new(db);
    let product = Product {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Product Test".to_owned(),
        price: 25.0,
        status: "disabled".to_owned(),
    };
    //
    let product_enabled = product.enable().unwrap();
    //
    let product_result = product_db.save(Box::new(product)).unwrap();
    assert_eq!(product_result.get_name(), "Product Test".to_owned());
    assert_eq!(product_result.get_price(), 25.0);
    assert_eq!(product_result.get_status(), "disabled".to_owned());
    //
    let db2 = setup().expect("open db connection error");
    let product_db = ProductDb::new(db2);
    let product_result = product_db.save(product_enabled).unwrap();
    assert_eq!(product_result.get_name(), "Product Test".to_owned());
    assert_eq!(product_result.get_price(), 25.0);
    assert_eq!(product_result.get_status(), "enabled".to_owned());
}
