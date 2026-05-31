//! Meta-schema tests. Grows into the property/contract suite (design-plan §7).
//! The `#[ignore]`d tests are skeletons — un-ignore them as `schema::validate`
//! is implemented in P1.

use schema_forge::schema;

#[test]
fn fixture_is_valid_json() {
    let raw = include_str!("../fixtures/customer-order.json");
    let _: serde_json::Value = serde_json::from_str(raw).expect("fixture is valid JSON");
}

#[test]
#[ignore = "P1: implement schema::validate"]
fn customer_order_fixture_is_valid() {
    let raw = include_str!("../fixtures/customer-order.json");
    let candidate: serde_json::Value = serde_json::from_str(raw).unwrap();
    let parsed = schema::validate(candidate).expect("fixture validates");
    assert_eq!(parsed.version, 1);
}

#[test]
#[ignore = "P1: implement schema::validate"]
fn relation_to_unknown_entity_is_rejected() {
    let candidate = serde_json::json!({
        "version": 1,
        "entities": {
            "Order": { "relations": { "customer": { "kind": "many-to-one", "target": "Customer" } } }
        }
    });
    let err = schema::validate(candidate).expect_err("must reject unknown relation target");
    assert!(err.to_string().contains("unknown entity"));
}
