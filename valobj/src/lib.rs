mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// Attribute macro to define a value object struct with optional validation, normalization, and (de)serialization.
///
/// Configuration options:
/// - `Normalize`: Calls a `normalize` function on the inner value before creating the value object. Requires the user to implement `Normalize<T>` for the value object.
/// - `Validate`: Calls a `validate` function on the inner value before creating the value object. Requires the user to implement `Validate<T>` for the value object. If validation fails, the `TryFrom` implementation will return an error instead of creating the value object.
///
/// Example usage:
/// ```
/// use valobj::{value_object, Normalize, Validate};
///
/// #[value_object(Normalize, Validate)]
/// pub struct OrderId(i32);
///
/// impl Validate<i32> for OrderId {
///     fn validate(value: &i32) -> Result<(), valobj::Error> {
///         if *value > 0 {
///             Ok(())
///         } else {
///             Err(valobj::Error::InvalidValue(
///                 "OrderId must be greater than 0".to_string(),
///             ))
///         }
///     }
/// }
///
/// impl Normalize<i32> for OrderId {
///     fn normalize(value: i32) -> i32 {
///         value * 10
///     }
/// }
/// ```
pub use valobj_macros::value_object;


/// Trait for validating a value when a `value_object` is created.
/// Only called if the `Validate` option is enabled in the `#[value_object]` attribute.
/// Example usage:
/// ```
/// use valobj::{value_object, Validate};
///
/// #[value_object(Validate)]
/// pub struct OrderId(i32);
///
/// impl Validate<i32> for OrderId {
///     fn validate(value: &i32) -> Result<(), valobj::Error> {
///         if *value > 0 {
///             Ok(())
///         } else {
///             Err(valobj::Error::InvalidValue(
///                 "OrderId must be greater than 0".to_string(),
///             ))
///         }
///     }
/// }
/// ```
pub trait Validate<T> {
    fn validate(value: &T) -> std::result::Result<(), Error>;
}

/// Trait for normalizing a value when a `value_object` is created.
/// Only called if the `Normalize` option is enabled in the `#[value_object]` attribute.
/// Example usage:
/// ```
/// use valobj::{value_object, Normalize};
///
/// #[value_object(Normalize)]
/// pub struct OrderId(i32);
///
/// impl Normalize<i32> for OrderId {
///     fn normalize(value: i32) -> i32 {
///         value * 10
///     }
/// }
/// ```
pub trait Normalize<T> {
    fn normalize(value: T) -> T;
}
