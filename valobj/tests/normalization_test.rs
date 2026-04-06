use valobj::{Normalize, Validate, value_object};

// Test normalization with String type
#[value_object(Normalize)]
pub struct TrimmedName(String);

impl Normalize<String> for TrimmedName {
    fn normalize(value: String) -> String {
        value.trim().to_string()
    }
}

#[test]
fn test_normalize_string_trim_applied() {
    let result = TrimmedName::from("  Charlie  ".to_string());
    assert_eq!(result.as_ref(), "Charlie");
}

// Test normalization with primitive type (f64)
#[value_object(Normalize)]
pub struct ScaledPrice(f64);

impl Normalize<f64> for ScaledPrice {
    fn normalize(value: f64) -> f64 {
        (value * 100.0).round() / 100.0 // Scale to 2 decimal places
    }
}

#[test]
fn test_normalize_f64_scaling_applied() {
    let result = ScaledPrice::from(19.99999);
    assert_eq!(result.get(), 20.0);
}

// Test combined Normalize and Validate
#[value_object(Normalize, Validate)]
pub struct NormalizedPositivePrice(f64);

impl Normalize<f64> for NormalizedPositivePrice {
    fn normalize(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }
}

impl Validate<f64> for NormalizedPositivePrice {
    fn validate(value: &f64) -> Result<(), valobj::Error> {
        if *value > 0.0 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "NormalizedPositivePrice must be greater than 0".to_string(),
            ))
        }
    }
}

#[test]
fn test_normalize_and_validate_f64_both_applied() {
    let result = NormalizedPositivePrice::try_from(9.999);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 10.0);
}

#[test]
fn test_normalize_and_validate_f64_validation_fails() {
    let result = NormalizedPositivePrice::try_from(0.0);
    assert!(result.is_err());
}
