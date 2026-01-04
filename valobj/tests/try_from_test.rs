use valobj::{value_object, Normalize, Validate};

// Test TryFrom trait with String type
#[value_object(Validate)]
pub struct PhoneNumber(String);

impl Validate<String> for PhoneNumber {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        let digits_only: String = value.chars().filter(|c| c.is_numeric()).collect();
        if digits_only.len() == 10 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "PhoneNumber must contain exactly 10 digits".to_string(),
            ))
        }
    }
}

#[test]
fn test_tryfrom_string_valid_phone_succeeds() {
    let result = PhoneNumber::try_from("555-123-4567".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_ref(), "555-123-4567");
}

#[test]
fn test_tryfrom_string_invalid_phone_fails() {
    let result = PhoneNumber::try_from("555-123-456".to_string());
    match result {
        Err(valobj::Error::InvalidValue(msg)) => {
            assert_eq!(msg, "PhoneNumber must contain exactly 10 digits");
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

// Test TryFrom trait with primitive type
#[value_object(Validate)]
pub struct Age(i32);

impl Validate<i32> for Age {
    fn validate(value: &i32) -> Result<(), valobj::Error> {
        if *value >= 0 && *value <= 150 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "Age must be between 0 and 150".to_string(),
            ))
        }
    }
}

#[test]
fn test_tryfrom_i32_valid_range_succeeds() {
    let result = Age::try_from(0i32);
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 0i32);
}

#[test]
fn test_tryfrom_i32_out_of_range_fails() {
    let result = Age::try_from(-5i32);
    match result {
        Err(valobj::Error::InvalidValue(msg)) => {
            assert_eq!(msg, "Age must be between 0 and 150");
        }
        _ => panic!("Expected InvalidValue error"),
    }
}

// Test TryFrom with both Normalize and Validate
#[value_object(Validate, Normalize)]
pub struct NormalizedUsername(String);

impl Normalize<String> for NormalizedUsername {
    fn normalize(value: String) -> String {
        value.trim().to_lowercase()
    }
}

impl Validate<String> for NormalizedUsername {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        if value.len() >= 3 && value.len() <= 20 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "NormalizedUsername must be 3-20 characters after normalization".to_string(),
            ))
        }
    }
}

#[test]
fn test_tryfrom_normalize_and_validate_both_applied() {
    let result = NormalizedUsername::try_from("  Alice  ".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_ref(), "alice");
}

#[test]
fn test_tryfrom_normalize_and_validate_too_short_fails() {
    let result = NormalizedUsername::try_from("  ab  ".to_string());
    assert!(result.is_err());
}

#[test]
fn test_tryfrom_normalize_and_validate_too_long_fails() {
    let result =
        NormalizedUsername::try_from("this_is_a_very_long_username_that_exceeds_limit".to_string());
    assert!(result.is_err());
}

#[test]
fn test_tryfrom_normalize_and_validate_error_message_correct() {
    let result = NormalizedUsername::try_from("xy".to_string());
    match result {
        Err(valobj::Error::InvalidValue(msg)) => {
            assert_eq!(
                msg,
                "NormalizedUsername must be 3-20 characters after normalization"
            );
        }
        _ => panic!("Expected InvalidValue error"),
    }
}
