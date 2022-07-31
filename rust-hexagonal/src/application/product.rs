use crate::adapters::db::product_db::DbError;
use error_stack;
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::fmt;
use std::{
    error::Error,
    fmt::{Debug, Display},
};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, PartialEq)]
pub struct ProductError(pub String);

impl Display for ProductError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl error_stack::Context for ProductError {}

#[cfg_attr(test, automock)]
pub trait ProductInterface {
    fn is_valid(&self) -> Result<bool, Box<ProductError>>;
    fn enable(&self) -> error_stack::Result<Box<dyn ProductInterface>, ProductError>;
    fn disable(&self) -> error_stack::Result<Box<dyn ProductInterface>, ProductError>;
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_status(&self) -> String;
    fn get_price(&self) -> f64;
}

#[derive(Debug, PartialEq, Validate, Default)]
pub struct Product {
    #[validate(custom(function = "validate_uuid", message = "invalid uuid-v4"))]
    pub id: String,
    #[validate(custom(
        function = "validate_name",
        message = "the name not must start with blank"
    ))]
    pub name: String,
    #[validate(range(min = 0, message = "the price must be greater or equal zero"))]
    pub price: f64,
    #[validate(custom(
        function = "validate_status",
        message = "the status must be enabled or disabled"
    ))]
    pub status: String,
}

pub const DISABLED: &str = "disabled";
pub const ENABLED: &str = "enabled";

impl Product {
    pub fn new() -> Product {
        Product {
            id: Uuid::new_v4().to_string(),
            status: DISABLED.to_owned(),
            // name: Default::default(),
            // price: Default::default(),
            // has the same effect as the above statement
            ..Default::default()
        }
    }
}

impl ProductInterface for Product {
    fn is_valid(&self) -> Result<bool, Box<ProductError>> {
        match self.validate() {
            Ok(_) => Ok(true),
            Err(e) => Err(Box::new(ProductError(format!("{e}")))),
        }
    }
    fn enable(&self) -> error_stack::Result<Box<dyn ProductInterface>, ProductError> {
        match self.price > 0.0 {
            true => {
                let product = Product {
                    id: self.id.to_owned(),
                    name: self.name.to_owned(),
                    price: self.price,
                    status: ENABLED.to_owned(),
                };
                Ok(Box::new(product))
            }
            false => Err(error_stack::Report::new(ProductError(
                "the price must be greater than zero to enable the product".into(),
            ))),
        }
    }
    fn disable(&self) -> error_stack::Result<Box<dyn ProductInterface>, ProductError> {
        match self.price != 0.0 {
            true => Err(error_stack::Report::new(ProductError(
                "the price must be zero in order to have the product disabled".into(),
            ))),
            false => {
                let product = Product {
                    id: self.id.to_owned(),
                    name: self.name.to_owned(),
                    price: self.price,
                    status: DISABLED.to_owned(),
                };
                Ok(Box::new(product))
            }
        }
    }
    fn get_id(&self) -> &str {
        self.id.as_str()
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_status(&self) -> String {
        let status = &self.status;
        status.to_string()
    }
    fn get_price(&self) -> f64 {
        self.price
    }
}

impl Debug for dyn ProductInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Product")
            .field("id", &self.get_id())
            .field("name", &self.get_name())
            .field("price", &self.get_price())
            .field("status", &self.get_status())
            .finish()
    }
}

#[cfg_attr(test, automock)]
pub trait ProductPersistanceInterface {
    fn get(&self, id: &str) -> error_stack::Result<Box<dyn ProductInterface>, DbError>;
    fn save(
        &self,
        product: Box<dyn ProductInterface>,
    ) -> error_stack::Result<Box<dyn ProductInterface>, DbError>;
}
pub trait ProductServiceInterface {
    fn get(id: &str) -> Result<Box<dyn ProductInterface>, Box<dyn Error>>;
    fn create(name: &str, price: f64) -> Result<Box<dyn ProductInterface>, Box<dyn Error>>;
    fn enable(product: dyn ProductInterface) -> Result<Box<dyn ProductInterface>, Box<dyn Error>>;
    fn disable(product: dyn ProductInterface) -> Result<Box<dyn ProductInterface>, Box<dyn Error>>;
}
fn validate_uuid(id: &str) -> Result<(), ValidationError> {
    let uuid = Uuid::parse_str(id);
    match uuid {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid uuid-v4")),
    }
}
fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "enabled" | "disabled" => Ok(()),
        _ => Err(ValidationError::new("enabled or disabled")),
    }
}
fn validate_name(name: &str) -> Result<(), ValidationError> {
    match name.starts_with(' ') {
        true => Err(ValidationError::new("not must blank")),
        false => Ok(()),
    }
}
