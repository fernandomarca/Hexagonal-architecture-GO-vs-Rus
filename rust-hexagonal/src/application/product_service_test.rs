use error_stack::ResultExt;
use mockall::predicate::eq;

use super::{
    product::{MockProductInterface, MockProductPersistanceInterface, Product, ProductError},
    product_service::ProductService,
};

#[test]
fn test_product_service_get() {
    let mut persistance = Box::new(MockProductPersistanceInterface::new());
    let _product = Box::new(MockProductInterface::new());
    //
    persistance
        .expect_get()
        .once()
        .with(eq("123"))
        .returning_st(|_x| Ok(Box::new(MockProductInterface::new())));
    //
    let service = ProductService { persistance };
    let _result = service.persistance.get("123").unwrap();
}

#[test]
fn test_product_service_create() {
    let product = Box::new(MockProductInterface::new());
    let mut persistance = Box::new(MockProductPersistanceInterface::new());
    //
    persistance.expect_save().once().returning_st(|p| Ok(p));
    //
    let service = ProductService { persistance };

    let p = Box::new(*product);
    let _result = service.persistance.save(p);
}

#[test]
fn test_product_service_enable() {
    use ::pretty_assertions::assert_eq;
    let mut persistance = Box::new(MockProductPersistanceInterface::new());
    persistance.expect_save().times(2).returning_st(|p| Ok(p));
    //
    let service = ProductService { persistance };
    //
    let product = Product {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Product Test".to_owned(),
        price: 10.0,
        status: "disabled".to_owned(),
    };
    let product_result = service
        .persistance
        .save(Box::new(product))
        .change_context(ProductError("save error".to_owned()));
    //
    let product_enabled = service.enable(product_result.unwrap().as_ref());
    assert_eq!(product_enabled.unwrap(), "enabled".to_owned());
}

#[test]
fn test_product_service_disable() {
    use ::pretty_assertions::assert_eq;
    let mut persistance = Box::new(MockProductPersistanceInterface::new());
    persistance.expect_save().times(2).returning_st(|p| Ok(p));
    //
    let service = ProductService { persistance };
    //
    let product = Product {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Product Test".to_owned(),
        price: 0.0,
        status: "enabled".to_owned(),
    };
    let product_result = service
        .persistance
        .save(Box::new(product))
        .change_context(ProductError("save error".to_owned()));
    //
    let product_enabled = service.disable(product_result.unwrap().as_ref());
    assert_eq!(product_enabled.unwrap(), "disabled".to_owned());
}
