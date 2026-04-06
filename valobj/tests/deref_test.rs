use valobj::value_object;

// Test Deref trait with primitive type (u64)
#[value_object]
pub struct UserId(u64);

#[test]
fn test_deref_u64_dereference_succeeds() {
    let user_id = UserId::from(12345u64);
    let deref_value: u64 = *user_id;
    assert_eq!(deref_value, 12345u64);
}

// Test Deref trait with String type
#[value_object]
pub struct Username(String);

#[test]
fn test_deref_string_str_reference_works() {
    let username = Username::from("alice".to_string());
    let deref_value: &str = &*username;
    assert_eq!(deref_value, "alice");
}

#[test]
fn test_deref_string_string_methods_available() {
    let username = Username::from("charlie".to_string());
    assert_eq!(username.len(), 7);
}
