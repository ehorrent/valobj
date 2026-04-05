use valobj::{value_object, Validate};

// Test validation with primitive type (i32)
#[value_object(Validate)]
pub struct PositiveInteger(i32);

impl Validate<i32> for PositiveInteger {
    fn validate(value: &i32) -> Result<(), valobj::Error> {
        if *value > 0 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "PositiveInteger must be greater than 0".to_string(),
            ))
        }
    }
}

#[test]
fn test_validate_i32_positive_succeeds() {
    let result = PositiveInteger::try_from(42i32);
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 42i32);
}

#[test]
fn test_validate_i32_zero_fails() {
    let result = PositiveInteger::try_from(0i32);
    assert!(result.is_err());
}

#[test]
fn test_validate_i32_negative_fails() {
    let result = PositiveInteger::try_from(-10i32);
    assert!(result.is_err());
}

#[test]
fn test_validate_i32_error_message_correct() {
    let result = PositiveInteger::try_from(-5i32);
    match result {
        Err(valobj::Error::InvalidValue(msg)) => {
            assert_eq!(msg, "PositiveInteger must be greater than 0");
        }
        _ => panic!("Expected InvalidValue error with specific message"),
    }
}

// Test validation with String type
#[value_object(Validate)]
#[derive(Debug)]
pub struct ValidEmail(String);

impl Validate<String> for ValidEmail {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        if value.contains('@') && value.contains('.') {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "ValidEmail must contain '@' and '.' characters".to_string(),
            ))
        }
    }
}

#[test]
fn test_validate_string_valid_format_succeeds() {
    let result = ValidEmail::try_from("user@example.com".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_validate_string_missing_at_sign_fails() {
    let result = ValidEmail::try_from("userexample.com".to_string());
    assert!(result.is_err());
}

#[test]
fn test_validate_string_missing_dot_fails() {
    let result = ValidEmail::try_from("user@examplecom".to_string());
    assert!(result.is_err());
}

#[test]
fn test_validate_string_error_message_correct() {
    let result = ValidEmail::try_from("invalid-email".to_string());
    match result {
        Err(valobj::Error::InvalidValue(msg)) => {
            assert_eq!(msg, "ValidEmail must contain '@' and '.' characters");
        }
        _ => panic!("Expected InvalidValue error with specific message"),
    }
}

