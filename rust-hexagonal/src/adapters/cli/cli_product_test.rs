use mockall::predicate::eq;

use crate::{
    adapters::cli::cli_product,
    application::product::{MockProductServiceInterface, Product},
};

#[test]
fn cli_product_test_create() {
    use pretty_assertions::assert_eq;
    let product_id = "id-test".to_string();
    let product_name = "Product Test".to_owned();
    let product_price = 25.99;
    let product_status = "enabled".to_owned();
    //create
    let mut service_mock = MockProductServiceInterface::new();
    service_mock
        .expect_create()
        .once()
        .with(eq("Product Test"), eq(25.99))
        .returning(|_s, _f| {
            Ok(Box::new(Product {
                id: "id-test".to_string(),
                name: "Product Test".to_owned(),
                price: 25.99,
                status: "enabled".to_owned(),
            }))
        });
    //
    let result_create_expected = format!(
        "Product ID {} with the name {} has been created with the price {} with status {}",
        product_id, product_name, product_price, product_status,
    );
    let result_create =
        cli_product::run(Box::new(service_mock), "create", "Product Test", 25.99, "").unwrap();
    assert_eq!(result_create_expected, result_create);
}

#[test]
fn cli_product_test_get() {
    use pretty_assertions::assert_eq;

    let product_id = "id-test".to_string();
    let product_name = "Product Test".to_owned();
    let product_price = 25.99;
    let product_status = "enabled".to_owned();
    //
    let mut service_mock = MockProductServiceInterface::new();
    //get
    service_mock
        .expect_get()
        .once()
        .with(eq("id-test".to_owned()))
        .returning_st(|_| {
            Ok(Box::new(Product {
                id: "id-test".to_string(),
                name: "Product Test".to_owned(),
                price: 25.99,
                status: "enabled".to_owned(),
            }))
        });
    //
    let result_create_expected = format!(
        "Product ID: {}\nName: {}\n Price: {}\n Status: {}",
        product_id, product_name, product_price, product_status,
    );
    let result_create =
        cli_product::run(Box::new(service_mock), "get", "", 0.0, "id-test").unwrap();
    //
    assert_eq!(result_create_expected, result_create);
}

#[test]
fn cli_product_test_enable() {
    use pretty_assertions::assert_eq;
    //
    let mut service_mock = MockProductServiceInterface::new();
    //get
    service_mock
        .expect_get()
        .once()
        .with(eq("id-test".to_owned()))
        .returning_st(|_| {
            Ok(Box::new(Product {
                id: "id-test".to_string(),
                name: "Product Test".to_owned(),
                price: 25.99,
                status: "enabled".to_owned(),
            }))
        });
    //enable
    service_mock
        .expect_enable()
        .once()
        .returning(|_s| Ok("enabled".to_owned()));
    //
    let result_create_expected = "Product has been enabled.".to_owned();
    let result_create =
        cli_product::run(Box::new(service_mock), "enable", "", 0.0, "id-test").unwrap();
    //
    assert_eq!(result_create_expected, result_create);
}

#[test]
fn cli_product_test_disable() {
    use pretty_assertions::assert_eq;
    //
    let mut service_mock = MockProductServiceInterface::new();
    //get
    service_mock
        .expect_get()
        .once()
        .with(eq("id-test".to_owned()))
        .returning_st(|_| {
            Ok(Box::new(Product {
                id: "id-test".to_string(),
                name: "Product Test".to_owned(),
                price: 25.99,
                status: "enabled".to_owned(),
            }))
        });
    //
    //disable
    service_mock
        .expect_disable()
        .once()
        .returning(|_s| Ok("disabled".to_owned()));
    //
    let result_create_expected = "Product has been disabled.".to_owned();
    let result_create =
        cli_product::run(Box::new(service_mock), "disable", "", 0.0, "id-test").unwrap();
    //
    assert_eq!(result_create_expected, result_create);
}
