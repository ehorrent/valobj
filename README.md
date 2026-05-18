# valobj (Value Objects)

[![build](https://github.com/ehorrent/valobj/actions/workflows/ci-quality.yml/badge.svg)](https://github.com/ehorrent/valobj/actions/workflows/ci-quality.yml)

Minimal improvements on the _newtype pattern_.

## Goal

_Value objects_ are a common design pattern across programming languages, and Rust's newtype pattern
provides a straightforward way to implement them. However, enforcing validity constraints on these
objects typically requires significant boilerplate code.

This crate aims to reduce this boilerplate by providing a lightweight **macro attribute** that
extends the newtype pattern with validation and normalization capabilities. The macro automatically
enforces domain invariants at construction time, enabling you to guarantee that **only valid values
can exist within your domain**.

```rust
use valobj::{value_object, Validate};

#[value_object(Normalize, Validate)]
pub struct Email(String);

impl Normalize<String> for Email {
    fn normalize(value: String) -> String {
        value.trim().to_string()
    }
}

impl Validate<String> for Email {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        if value.contains('@') && value.contains('.') {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue(
                "Email must contain '@' and '.' characters".to_string(),
            ))
        }
    }
}

fn main() {
    // try_from will normalize the input value and validate it, ensuring that only valid emails can be created
    if let Ok(email) = Email::try_from("USER@example.com".to_string()) {
        assert_eq!(email.as_ref(), "user@example.com");
    }
}
``` 

### Primitive obsession

Creating new types that wrap primitives helps you
avoid [Primitive obsession](https://senthilnayagan.net/blog/tech/primitive-obsession/), a code smell
where primitive types like String or u64 are used directly to represent domain concepts without
meaningful constraints or semantics.

## When to use value objects?

If you already use newtype pattern, you can consider using `value_object` attribute when you need to
**normalize** or **validate** the inner value, or when you want to define invariants easily (e.g.
ensure a
`UserId` is always positive or that a `Username` is not empty...).

## Reference

### Construction

Value objects can be constructed with either `from` or `try_from` methods, depending on whether
validation is enabled or not.

### Getter

To maintain consistency, the tuple is immutable and you cannot access the `.0` field directly.
To get the inner value, a `get` method is generated, which returns a copy (or a `&str`
in case of string) to the inner value:

```rust
#[valobj::value_object]
pub struct UserId(u64);

fn main() {
    let user_id = UserId::from(1);
    let value = user_id.0; // This will not compile
    let value = user_id.get(); // This will work
}
```

Some other traits are also implemented to allow easy access:

- `AsRef<T>` (`AsRef<str>` in case of a `String` type)
- `Deref<Target=T>`

```rust
#[valobj::value_object]
pub struct UserId(u64);

fn main() {
    let user_id = UserId::from(1);
    let value = *user_id;
}
```

### Validation

You can define a validation function that checks if the input value meets some requirements. If
the `Validate` trait is implemented, a `TryFrom` implementation will be generated,
allowing you to create value objects from the inner type while ensuring that the value is valid.

To enable validation, you need to :

- Add `Validate` attribute to the macro: `#[valobj::value_object(Validate)]`
- Implement the `Validate` trait for the inner type of your value object.

```rust
pub trait Validate<T> {
    fn validate(value: &T) -> std::result::Result<(), Error>;
} 
```

### Normalization

You can define a normalization function that transforms the input value into a
canonical form (e.g. trim whitespaces from a string, change it to lowercase...).

To enable normalization, you need to :

- Add `Normalize` attribute to the macro: `#[valobj::value_object(Normalize)]`
- Implement the `Normalize` trait for the inner type of your value object.

The `normalize` method should return the normalized value.

```rust
pub trait Normalize<T> {
    fn normalize(value: T) -> T;
}
```

#