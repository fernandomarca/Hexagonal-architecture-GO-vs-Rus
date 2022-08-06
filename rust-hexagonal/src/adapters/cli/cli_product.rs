use std::fmt::Display;

use error_stack::{Context, ResultExt};

use crate::application::product::ProductServiceInterface;

#[derive(Debug)]
pub struct CliError(pub String);

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cli error")
    }
}

impl Context for CliError {}
pub fn run(
    service: Box<dyn ProductServiceInterface>,
    action: &str,
    product_name: &str,
    price: f64,
    product_id: &str,
) -> error_stack::Result<String, CliError> {
    let result = match action {
        "create" => {
            println!("create into: {},{}", product_name, price);
            let product = service
                .create(product_name, price)
                .change_context(CliError("create command cli error".to_owned()))?;
            format!(
                "Product ID {} with the name {} has been created with the price {} with status {}",
                product.get_id(),
                product.get_name(),
                product.get_price(),
                product.get_status()
            )
        }
        "enable" => {
            let product = service
                .get(product_id.to_string())
                .change_context(CliError("get product command cli error".to_owned()))?;
            let res = service
                .enable(product)
                .change_context(CliError("enable command cli error".to_owned()))?;
            format!("Product {} has been enabled.", res)
        }
        "disable" => {
            let product = service
                .get(product_id.to_string())
                .change_context(CliError("get product command cli error".to_owned()))?;
            let res = service
                .disable(product)
                .change_context(CliError("enable command cli error".to_owned()))?;
            format!("Product {} has been disabled.", res)
        }
        "get" => {
            let res = service
                .get(product_id.to_string())
                .change_context(CliError("get product command cli error".to_owned()))?;
            format!(
                "Product ID: {}\nName: {}\n Price: {}\n Status: {}",
                res.get_id(),
                res.get_name(),
                res.get_price(),
                res.get_status(),
            )
        }
        _ => {
            Err(CliError("try again: action not exist".to_string()))?;
            "".to_string()
        }
    };
    Ok(result)
}
