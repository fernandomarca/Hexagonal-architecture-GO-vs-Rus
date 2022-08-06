#![allow(dead_code)]

use super::product::{
    Product, ProductError, ProductInterface, ProductPersistanceInterface, ProductServiceInterface,
};
use crate::adapters::db::product_db::DbError;
use error_stack::{self, ResultExt};

pub struct ProductService {
    pub persistance: Box<dyn ProductPersistanceInterface>,
}

impl ProductService {
    pub fn new(persistance: Box<dyn ProductPersistanceInterface>) -> ProductService {
        ProductService { persistance }
    }
}

impl ProductServiceInterface for ProductService {
    fn get(&self, id: String) -> error_stack::Result<Box<dyn ProductInterface>, DbError> {
        self.persistance.get(&id)
    }
    fn create(
        &self,
        name: &str,
        price: f64,
    ) -> error_stack::Result<Box<dyn ProductInterface>, DbError> {
        let mut product = Product::new();
        product.name = name.to_string();
        product.price = price;
        self.persistance.save(Box::new(product))
    }
    fn enable(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> error_stack::Result<String, ProductError> {
        let result = product.enable()?;
        let product_result = self
            .persistance
            .save(result)
            .change_context(ProductError("save error".to_owned()))?;
        let status = product_result.get_status();
        Ok(status)
    }
    fn disable(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> error_stack::Result<String, ProductError> {
        let result = product.disable()?;
        let product_result = self
            .persistance
            .save(result)
            .change_context(ProductError("save error".to_owned()))?;
        let status = product_result.get_status();
        Ok(status)
    }
}
