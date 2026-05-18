# valobj (Value Objects)

[![build](https://github.com/ehorrent/valobj/actions/workflows/ci-quality.yml/badge.svg)](https://github.com/ehorrent/valobj/actions/workflows/ci-quality.yml)

Minimal improvements on the _newtype pattern_, to create _value objects_ on top of primitive types,
with validation and normalization capabilities.

## Installation

Add valobj to your `Cargo.toml`:

```toml
[dependencies]
valobj = "0.1"
```

## Quick Start

Create a validated value object in 3 steps:

1. **Add the macro attribute** to your struct
2. **Implement validation or normalization traits** (optional)
3. **Construct using `from()` or `try_from()`**

```rust
use valobj::value_object;

#[value_object]
pub struct UserId(u64);

fn main() {
    let user_id = UserId::from(42);
    assert_eq!(user_id.get(), 42);
}
```

For validation:

```rust
use valobj::{value_object, Validate};

#[value_object(Validate)]
pub struct Age(u32);

impl Validate<u32> for Age {
    fn validate(value: &u32) -> Result<(), valobj::Error> {
        if *value <= 150 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue("Age must be <= 150".into()))
        }
    }
}

fn main() {
    match Age::try_from(25u32) {
        Ok(age) => println!("Valid age: {}", age.get()),
        Err(e) => println!("Invalid: {}", e),
    }
}
```

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

If you already use newtype pattern, consider using `value_object` attribute when you need to:

- **Normalize** values (trim strings, clamp numbers, etc.)
- **Validate** inputs at construction time
- **Define invariants** easily (e.g., ensure a `UserId` is always positive, `Username` is not empty)
- **Prevent type confusion** by wrapping primitives with semantic meaning
- **Enforce domain rules** at the type level

### Real-World Use Cases

**UserId** — Ensure user IDs are always positive:
```rust
#[valobj::value_object(Validate)]
pub struct UserId(u64);

impl valobj::Validate<u64> for UserId {
    fn validate(value: &u64) -> Result<(), valobj::Error> {
        if *value > 0 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue("UserId must be positive".into()))
        }
    }
}
```

**Email** — Validate format and normalize to lowercase:
```rust
#[valobj::value_object(Normalize, Validate)]
pub struct Email(String);

impl valobj::Normalize<String> for Email {
    fn normalize(value: String) -> String {
        value.trim().to_lowercase()
    }
}

impl valobj::Validate<String> for Email {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        if value.contains('@') && value.contains('.') {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue("Invalid email format".into()))
        }
    }
}
```

**PhoneNumber** — Strip formatting and validate length:
```rust
#[valobj::value_object(Normalize, Validate)]
pub struct PhoneNumber(String);

impl valobj::Normalize<String> for PhoneNumber {
    fn normalize(value: String) -> String {
        value.chars().filter(|c| c.is_numeric()).collect()
    }
}

impl valobj::Validate<String> for PhoneNumber {
    fn validate(value: &String) -> Result<(), valobj::Error> {
        if value.len() == 10 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue("Phone must have 10 digits".into()))
        }
    }
}
```

## Reference

### Construction

Value objects can be constructed with either `from` or `try_from` methods, depending on whether
validation is enabled or not.

- **`from(value)`** — Infallible construction (always succeeds). Use when no validation is needed.
- **`try_from(value)`** — Fallible construction (returns `Result`). Use with validation enabled.

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

Validation ensures that only valid values can be created. If the `Validate` trait is implemented, a `TryFrom` implementation will be generated.

**When to use:** Enforce domain invariants (positive numbers, non-empty strings, valid email formats, etc.)

**Setup:**
1. Add `Validate` attribute to the macro: `#[valobj::value_object(Validate)]`
2. Implement the `Validate` trait for your type

**Trait definition:**
```rust
pub trait Validate<T> {
    fn validate(value: &T) -> std::result::Result<(), Error>;
}
```

**Usage:**
```rust
use valobj::{value_object, Validate};

#[value_object(Validate)]
pub struct Score(i32);

impl Validate<i32> for Score {
    fn validate(value: &i32) -> Result<(), valobj::Error> {
        if *value >= 0 && *value <= 100 {
            Ok(())
        } else {
            Err(valobj::Error::InvalidValue("Score must be 0-100".into()))
        }
    }
}

fn main() {
    // Returns Ok if valid, Err if invalid
    let score = Score::try_from(85)?;
}
```

### Normalization

Normalization transforms input into a canonical form before validation. Useful for cleaning up user input.

**When to use:** Trim whitespace, convert case, parse formats, clamp values, etc.

**Setup:**
1. Add `Normalize` attribute to the macro: `#[valobj::value_object(Normalize)]`
2. Implement the `Normalize` trait for your type

**Trait definition:**
```rust
pub trait Normalize<T> {
    fn normalize(value: T) -> T;
}
```

**Usage:**
```rust
use valobj::{value_object, Normalize};

#[value_object(Normalize)]
pub struct Username(String);

impl Normalize<String> for Username {
    fn normalize(value: String) -> String {
        value.trim().to_lowercase()
    }
}

fn main() {
    let username = Username::from("  Alice  ".to_string());
    assert_eq!(username.as_ref(), "alice");
}
```

**Order of operations:** When both are enabled, normalization happens first, then validation:
```rust
#[value_object(Normalize, Validate)]
pub struct Username(String);
// 1. normalize() is called first
// 2. validate() is called on the normalized value
```

## FAQ

**Q: Why use value objects instead of type aliases?**

A: Type aliases don't provide any safety or encapsulation. `type UserId = u64` and `type PostId = u64` are interchangeable and would pass the same value to the wrong function. Value objects wrap the type and prevent this confusion:

```rust
#[valobj::value_object]
pub struct UserId(u64);

#[valobj::value_object]
pub struct PostId(u64);

fn get_user(id: UserId) { }  // Won't accept PostId
fn get_post(id: PostId) { }   // Won't accept UserId
```

**Q: What traits are automatically derived?**

A: The macro generates implementations for:
- `From<T>` — Construct without validation
- `TryFrom<T>` — Construct with validation (if `Validate` is enabled)
- `AsRef<T>` — Borrow inner value
- `Deref<Target=T>` — Dereference to inner value
- `get()` method — Retrieve a copy/reference to inner value
- `Debug`, `Clone`, `Copy` (where applicable)

**Q: Do value objects have runtime overhead?**

A: No. Value objects are zero-cost abstractions. In release builds, they compile down to the same code as using primitives directly. The wrapper is purely a compile-time construct.

**Q: Can I use value objects with custom types?**

A: Currently, the macro works with single-field tuple structs. The inner type can be any Rust type (primitive, String, custom structs, etc.).

**Q: How do I handle errors from validation?**

A: Use `try_from()` which returns a `Result`. The error variant is `valobj::Error::InvalidValue(String)` containing your error message:

```rust
match Email::try_from(input) {
    Ok(email) => println!("Valid: {}", email.as_ref()),
    Err(valobj::Error::InvalidValue(msg)) => println!("Error: {}", msg),
}
```

**Q: Can I have multiple value objects in the same module without conflicts?**

A: Yes. Each value object is independent. They can coexist and have separate validation/normalization logic.

**Q: Does this work with serialization (serde)?**

A: The macro generates newtype wrappers compatible with serde. You can use `#[derive(Serialize, Deserialize)]` on value objects. Serialization respects the newtype structure.
