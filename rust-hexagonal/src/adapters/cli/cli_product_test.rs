use std::fmt::format;

use mockall::predicate::eq;

use crate::{
    adapters::cli::cli_product,
    application::product::{
        MockProductInterface, MockProductServiceInterface, Product, ProductInterface,
    },
};

// #[test]
// fn cli_product_test() {
//     use pretty_assertions::assert_eq;

//     // let product = Product {
//     //     id: "id-test".to_string(),
//     //     name: "Product Test".to_owned(),
//     //     price: 25.99,
//     //     status: "enabled".to_owned(),
//     // };
//     let product_id = "id-test".to_string();
//     let product_price = 25.99;
//     let product_status = "enabled".to_string();
//     let product_name = "name-test".to_string();

//     let mut product_mock = MockProductInterface::new();
//     product_mock.expect_get_id().once().return_const(product_id);
//     //
//     product_mock
//         .expect_get_name()
//         .once()
//         .return_const(product_name);
//     //
//     product_mock
//         .expect_get_price()
//         .once()
//         .return_const(product_price);
//     //
//     product_mock
//         .expect_get_status()
//         .once()
//         .return_const(product_status);
//     //create
//     let mut service_mock = MockProductServiceInterface::new();
//     service_mock
//         .expect_create()
//         .once()
//         .with(eq("Product Test"), eq(product_price))
//         .returning_st(|_s, _f| Ok(Box::new(MockProductInterface::new())));
//     //get
//     service_mock
//         .expect_get()
//         .once()
//         .with(eq("id-test"))
//         .returning_st(|_| Ok(Box::new(MockProductInterface::new())));
//     //enable
//     service_mock
//         .expect_enable()
//         .once()
//         .returning_st(|_| Ok(Box::new(MockProductInterface::new())));
//     //disable
//     service_mock
//         .expect_disable()
//         .once()
//         .returning_st(|_| Ok(Box::new(MockProductInterface::new())));
//     //
//     // let result_create_expected = format!(
//     //     "Product ID {} with the name {} has been created with the price {} with status {}",
//     //     product_mock.get_id(),
//     //     product_mock.get_name(),
//     //     product_mock.get_price(),
//     //     product_mock.get_status(),
//     // );
//     let result_create = cli_product::run(
//         Box::new(service_mock),
//         "create",
//         "Product Test",
//         product_price,
//         "id-test",
//     )
//     .unwrap();
//     println!("{}", result_create);
//     // assert_eq!(result_create_expected, result_create);
// }

#[test]
fn cli_product_test_create() {
    use pretty_assertions::assert_eq;

    let product = Product {
        id: "id-test".to_string(),
        name: "Product Test".to_owned(),
        price: 25.99,
        status: "enabled".to_owned(),
    };
    let product_id = "id-test".to_string();
    let product_price = 25.99;
    let product_status = "enabled".to_string();
    let product_name = "Product Test".to_string();

    let mut product_mock = MockProductInterface::new();
    product_mock
        .expect_get_id()
        .once()
        .return_const("id-test".to_string());
    //
    product_mock
        .expect_get_name()
        .once()
        .return_const("Product Test".to_string());
    //
    product_mock.expect_get_price().once().return_const(25.99);
    //
    product_mock
        .expect_get_status()
        .once()
        .return_const("enabled".to_string());
    //create
    let mut service_mock = MockProductServiceInterface::new();
    service_mock
        .expect_create()
        .once()
        .with(eq("Product Test"), eq(25.99))
        .returning(|_s, _f| Ok(Box::new(MockProductInterface::new())));
    //
    // let result_create_expected = format!(
    //     "Product ID {} with the name {} has been created with the price {} with status {}",
    //     product.get_id(),
    //     product.get_name(),
    //     product.get_price(),
    //     product.get_status(),
    // );
    let result_create = cli_product::run(
        Box::new(service_mock),
        "create",
        "Product Test",
        25.99,
        "id-test",
    )
    .unwrap();
    println!("service:print {:?}", result_create);
    // assert_eq!(result_create_expected, result_create);
}
