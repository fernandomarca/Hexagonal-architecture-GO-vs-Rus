#![allow(dead_code)]

use std::fmt::Display;

use crate::application::product::{Product, ProductInterface, ProductPersistanceInterface};
use error_stack::{Context, Report, Result};

use rusqlite::{Connection, OptionalExtension};

#[derive(Debug)]
pub struct DbError;

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("open connection db error")
    }
}

impl Context for DbError {}

pub struct ProductDb {
    db: Connection,
}

impl ProductPersistanceInterface for ProductDb {
    fn get(&self, id: &str) -> Result<Box<dyn ProductInterface>, DbError> {
        let sql = "select id, name, price, status from products where id=?";
        let mut statement = self.db.prepare(sql).or_else(|_o| {
            return Err(
                Report::new(DbError).attach_printable(format!("{sql} sql cannot be converted"))
            );
        })?;
        let product = statement.query_row([id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                status: row.get(3)?,
            })
        });
        let result = product.or_else(|_o| {
            let msg = format!("{sql} sql  no results are returned");
            Err(Report::new(DbError).attach_printable(msg))
        })?;
        Ok(Box::new(result))
    }
    fn save(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> Result<Box<dyn ProductInterface>, DbError> {
        let result = self
            .db
            .query_row(
                "select id, name, price, status from products where id=?",
                [product.get_id()],
                |row| {
                    Ok(Product {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        price: row.get(2)?,
                        status: row.get(3)?,
                    })
                },
            )
            .optional();
        match result {
            Ok(p) => {
                if p.is_some() {
                    self.update(product)
                } else {
                    self.create(product)
                }
            }
            Err(_) => Err(Report::new(DbError).attach_printable("save error")),
        }
    }
}

impl ProductDb {
    pub fn new(db: Connection) -> ProductDb {
        ProductDb { db }
    }
    fn create(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> Result<Box<dyn ProductInterface>, DbError> {
        let statement = self.db.prepare(
            "insert into products(id,name,price,status)
        values(?,?,?,?)",
        );
        let result = statement.map(|mut s| {
            s.execute((
                product.get_id(),
                product.get_name(),
                product.get_price(),
                product.get_status(),
            ))
        });
        match result {
            Ok(_) => Ok(product),
            Err(_) => Err(Report::new(DbError).attach_printable("create error")),
        }
    }
    fn update(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> Result<Box<dyn ProductInterface>, DbError> {
        // println!("enable?: {}", product.get_status());

        let result = self.db.execute(
            "update products set name = ?, price=?, status=? where id=?",
            (
                product.get_name(),
                product.get_price(),
                product.get_status(),
                product.get_id(),
            ),
        );
        match result {
            Ok(_) => Ok(product),
            Err(_) => Err(Report::new(DbError).attach_printable("update error")),
        }
    }
}
