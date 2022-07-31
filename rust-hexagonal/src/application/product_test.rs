use super::product::*;
fn product_factory() -> Product {
    // Product {
    //     id: Uuid::new_v4().to_string(),
    //     name: "Product1".to_owned(),
    //     price: 10.0,
    //     status: DISABLED.to_owned(),
    // }
    Product::new()
}

#[test]
fn test_product_enable() {
    use pretty_assertions::assert_eq;
    let mut product = product_factory();
    product.price = 10.0;
    assert_eq!(product.enable().unwrap().get_status(), "enabled".to_owned());
    product.price = 0.0;
    assert_eq!(
        product.enable().unwrap_err().to_string(),
        ProductError(
            "There is an error: the price must be greater than zero to enable the product".into()
        )
        .0
    );
}

#[test]
fn test_product_disable() {
    use pretty_assertions::assert_eq;
    let mut product = product_factory();
    product.status = ENABLED.to_owned();
    product.price = 0.0;

    assert_eq!(
        product.disable().unwrap().get_status(),
        "disabled".to_owned()
    );

    product.price = 10.0;
    assert_eq!(
        product.disable().unwrap_err().to_string(),
        ProductError(
            "There is an error: the price must be zero in order to have the product disabled"
                .into()
        )
        .0
    );
}

#[test]
fn test_product_is_valid() {
    use pretty_assertions::assert_eq;
    let mut product = product_factory();
    assert!(product.is_valid().unwrap());
    //
    product.status = "INVALID".to_owned();
    assert_eq!(
        product.is_valid(),
        Err(Box::new(ProductError(
            "status: the status must be enabled or disabled".into()
        )))
    );
    //
    product.status = ENABLED.to_owned();
    assert!(product.is_valid().unwrap());
    //
    product.price = -10.0;
    assert_eq!(
        product.is_valid(),
        Err(Box::new(ProductError(
            "price: the price must be greater or equal zero".into()
        )))
    );
}

#[test]
fn test_product_name() {
    let mut product = product_factory();
    product.name = "".to_owned();
    assert!(product.is_valid().unwrap());
    //
}

//this should passed or not?
// I can implement test for "not blank"
#[test]
fn test_product_name_not_must_start_with_blank() {
    use pretty_assertions::assert_eq;
    let mut product = product_factory();
    product.name = "    white_space_name".to_owned();
    assert_eq!(
        product.is_valid(),
        Err(Box::new(ProductError(
            "name: the name not must start with blank".into()
        )))
    );
    //
}
