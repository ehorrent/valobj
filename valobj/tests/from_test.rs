use valobj::{value_object, Normalize};

// Test From trait with String type (no validation, with normalization)
#[value_object(Normalize)]
pub struct ColorCode(String);

impl Normalize<String> for ColorCode {
    fn normalize(value: String) -> String {
        value.to_uppercase()
    }
}

#[test]
fn test_from_string_normalize_uppercase_applied() {
    let color = ColorCode::from("red".to_string());
    assert_eq!(color.as_ref(), "RED");
}

#[test]
fn test_from_string_mixed_case_normalized() {
    let color = ColorCode::from("BlUe".to_string());
    assert_eq!(color.as_ref(), "BLUE");
}

#[test]
fn test_from_string_already_uppercase_preserved() {
    let color = ColorCode::from("GREEN".to_string());
    assert_eq!(color.as_ref(), "GREEN");
}

// Test From trait with primitive type (no validation, with normalization)
#[value_object(Normalize)]
pub struct Percentage(f64);

impl Normalize<f64> for Percentage {
    fn normalize(value: f64) -> f64 {
        value.clamp(0.0, 100.0)
    }
}

#[test]
fn test_from_f64_clamp_above_max_works() {
    let pct = Percentage::from(150.0);
    assert_eq!(pct.get(), 100.0);
}

#[test]
fn test_from_f64_clamp_below_min_works() {
    let pct = Percentage::from(-50.0);
    assert_eq!(pct.get(), 0.0);
}

#[test]
fn test_from_f64_valid_value_preserved() {
    let pct = Percentage::from(75.5);
    assert_eq!(pct.get(), 75.5);
}

// Test From trait without normalization
#[value_object]
pub struct SimpleCounter(i32);

#[test]
fn test_from_i32_no_validation_value_stored() {
    let counter = SimpleCounter::from(42i32);
    assert_eq!(*counter, 42i32);
}

#[test]
fn test_from_i32_zero_allowed() {
    let counter = SimpleCounter::from(0i32);
    assert_eq!(*counter, 0i32);
}

#[test]
fn test_from_i32_negative_allowed() {
    let counter = SimpleCounter::from(-10i32);
    assert_eq!(*counter, -10i32);
}

// Test From trait with String (no normalization)
#[value_object]
pub struct Label(String);

#[test]
fn test_from_string_no_normalization_stored_as_is() {
    let label = Label::from("test label".to_string());
    assert_eq!(label.as_ref(), "test label");
}

#[test]
fn test_from_string_case_preserved() {
    let label = Label::from("MyLabel".to_string());
    assert_eq!(label.as_ref(), "MyLabel");
}

#[test]
fn test_from_string_empty_string_allowed() {
    let label = Label::from("".to_string());
    assert_eq!(label.as_ref(), "");
}

