use std::convert::AsRef;
use valobj::value_object;

// Test getter implementation with String type
#[value_object]
pub struct ApiKey(String);

#[test]
fn test_getter_string_get_returns_str_reference() {
    let key = ApiKey::from("sk-1234567890abcdef".to_string());
    let key_ref: &str = key.get();
    assert_eq!(key_ref, "sk-1234567890abcdef");
}

#[test]
fn test_getter_string_get_returns_correct_type() {
    let key = ApiKey::from("secret-key".to_string());
    let key_ref = key.get();
    // Verify it's &str by checking string methods are available
    assert_eq!(key_ref.len(), 10);
}

#[test]
fn test_getter_string_as_ref_returns_str() {
    let key = ApiKey::from("my-api-key".to_string());
    let key_ref: &str = key.as_ref();
    assert_eq!(key_ref, "my-api-key");
}

#[test]
fn test_getter_string_as_ref_trait_works() {
    let key = ApiKey::from("test-key".to_string());
    // Test that AsRef<str> is properly implemented
    fn requires_as_ref<T: AsRef<str>>(t: T) -> usize {
        t.as_ref().len()
    }
    assert_eq!(requires_as_ref(&key), 8);
}

#[test]
fn test_getter_string_get_and_as_ref_equivalent() {
    let key = ApiKey::from("identical-key".to_string());
    assert_eq!(key.get(), key.as_ref());
}

// Test getter implementation with primitive type (i32)
#[value_object]
pub struct Score(i32);

#[test]
fn test_getter_i32_get_returns_copy() {
    let score = Score::from(100i32);
    let value: i32 = score.get();
    assert_eq!(value, 100i32);
}

#[test]
fn test_getter_i32_get_copyable_multiple_calls() {
    let score = Score::from(42i32);
    let val1 = score.get();
    let val2 = score.get(); // Can call get() multiple times
    assert_eq!(val1, val2);
}

#[test]
fn test_getter_i32_as_ref_returns_reference() {
    let score = Score::from(75i32);
    let score_ref: &i32 = score.as_ref();
    assert_eq!(*score_ref, 75i32);
}

#[test]
fn test_getter_i32_as_ref_trait_works() {
    let score = Score::from(88i32);
    fn requires_as_ref<T: AsRef<i32>>(t: T) -> i32 {
        *t.as_ref()
    }
    assert_eq!(requires_as_ref(&score), 88i32);
}

#[test]
fn test_getter_i32_get_returns_value() {
    let score = Score::from(50i32);
    let value = score.get();
    // For primitives, get() returns the value directly
    assert_eq!(value, 50i32);
}

// Test getter with f64
#[value_object]
pub struct Rating(f64);

#[test]
fn test_getter_f64_get_returns_value() {
    let rating = Rating::from(4.5);
    let value: f64 = rating.get();
    assert_eq!(value, 4.5);
}

#[test]
fn test_getter_f64_as_ref_returns_reference() {
    let rating = Rating::from(3.7);
    let rating_ref: &f64 = rating.as_ref();
    assert_eq!(*rating_ref, 3.7);
}

#[test]
fn test_getter_f64_as_ref_trait_works() {
    let rating = Rating::from(2.9);
    fn requires_as_ref<T: AsRef<f64>>(t: T) -> f64 {
        *t.as_ref()
    }
    assert_eq!(requires_as_ref(&rating), 2.9);
}

// Test getter with u64
#[value_object]
pub struct Timestamp(u64);

#[test]
fn test_getter_u64_get_returns_value() {
    let ts = Timestamp::from(1234567890u64);
    let value: u64 = ts.get();
    assert_eq!(value, 1234567890u64);
}

#[test]
fn test_getter_u64_get_multiple_calls() {
    let ts = Timestamp::from(9876543210u64);
    assert_eq!(ts.get(), ts.get());
    assert_eq!(ts.get(), 9876543210u64);
}

#[test]
fn test_getter_u64_as_ref_returns_reference() {
    let ts = Timestamp::from(5555555555u64);
    let ts_ref: &u64 = ts.as_ref();
    assert_eq!(*ts_ref, 5555555555u64);
}
