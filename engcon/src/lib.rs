//! # EngCon
//!
//! EngCon (Engineering Contracts) is a set of macros and traits defining contracts often found in
//! engineering problems, e.g. the design of a distilation column.
//!
//! # Including EngCon in your Project
//!
//! Import engcon and engcon_macros into your project by adding the following lines to your Cargo.toml.
//! engcon_macros contains the macros needed to derive the traits in EngCon.
//!
//! ```toml
//! [dependencies]
//! engcon = "0.1"
//! engcon_macros = "0.1"
//!
//! # You can also access engcon_macros exports directly through strum using the "derive" feature
//! engcon = { version = "0.1", features = ["derive"] }
//! ```
//!
//! This pattern is also used by by the well known [strum crate](https://docs.rs/strum/latest/strum/) that has helpful procedural macros
//! for enumerations.
//!

use std::{
    error::Error,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "derive")]
pub use engcon_macros::*;

/// A new-type  that ensures validated data for a generic T.
///
/// Use the [Validatable] dervice macro and it's rules to
/// implement define validation rules on a type T.
///
/// The type parameter `T` must be [Sized] and implement the [Validator] trait.
pub struct Validated<T: Validator + Sized> {
    inner: T,
}

/// An error type that is used when a validation error occurs
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    msg: String,
    src: String,
}

/// Provides methods to validate and to transform into a [Validated] new-type.
///
/// Using the derive macro [Validatable] is recommended instead of a manual implemenation.
pub trait Validator: Sized {
    /// Checks if the underlying data is valid and returns an [ValidationError] if not.
    ///
    /// Using the derive macro [Validatable] is recommended instead of a
    /// manual implemenation.
    fn validate(&self) -> Result<(), ValidationError>;

    /// tries to transform Self into a [Validated] may give an [ValidationError]
    fn try_into_validated(self) -> Result<Validated<Self>, ValidationError> {
        match self.validate() {
            Ok(_) => {
                // just checked if that is validated...
                let reval = unsafe { Validated::new_unchecked(self) };
                Ok(reval)
            }
            Err(err) => Err(err),
        }
    }
}

impl Error for ValidationError {}

impl ValidationError {
    pub fn new(msg: String, src: String) -> Self {
        ValidationError { msg, src }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error validating '{}': {}", self.src, self.msg)
    }
}

impl<T: Validator + Sized> Validated<T> {
    /// Generates a validated instance of T, usable for compile-time API safety.
    ///
    /// # Safety
    /// The caller has to ensure Validator::validate returns true for that function
    pub unsafe fn new_unchecked(inner: T) -> Self {
        Validated::<T> { inner }
    }

    /// gets the inner unchecked type
    pub fn into_inner(self) -> T {
        self.inner
    }
}

/* TODO: I don't get where a Into<Validated<T>> has been implemented...
 *       All the codegen of TryFrom Into etc. was commented out once
 * error[E0119]: conflicting implementations of trait `std::convert::TryFrom<_>` for type `Validated<_>`
  --> engcon\src\lib.rs:69:1
   |
69 | impl<T: Validator + Sized> TryFrom<T> for Validated<T> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `core`:
           - impl<T, U> std::convert::TryFrom<U> for T
             where U: std::convert::Into<T>;


 */
/*
impl<T: Validator + Sized> TryFrom<T> for Validated<T> {
    type Error = ValidationError;

    fn try_from(value: T) -> Result<Self, Self::Error> {
        match value.validate() {
            Ok(_) => Ok(Validated::<T> { inner: value }),
            Err(err) => Err(err),
        }
    }
}
 */

impl<T: Validator + Sized> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Validator + Sized> DerefMut for Validated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {

    #[derive(Debug, Clone, PartialEq)]
    struct PlainOldData {
        only_lower: String,
    }

    impl Validator for PlainOldData {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.only_lower.chars().any(|ch| !ch.is_lowercase()) {
                Err(ValidationError::new(
                    "String is not entirely lower-case".to_owned(),
                    "Src".to_owned(),
                ))
            } else {
                Ok(())
            }
        }
    }

    use super::*;

    #[test]
    fn all_lowercase_works() {
        let tmp = PlainOldData {
            only_lower: "thisisonlylowercase".to_owned(),
        };
        let result = tmp.clone().validate();
        assert!(result.is_ok());
    }

    #[test]
    fn whitespaces_are_errors() {
        let tmp = PlainOldData {
            only_lower: "this is not only lowercase".to_owned(),
        };
        let result = tmp.clone().validate();
        assert!(result.is_err());
    }
}
