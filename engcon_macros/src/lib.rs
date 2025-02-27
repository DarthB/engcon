//! # EngCon
//!
//! EngCon is a set of macros and traits defining contracts often found in
//! engineering problems, e.g. the design of a distilation column.
//!
//! This crate only contains derive macros for use with the
//! [`engcon`](https://docs.rs/engcon)
//! crate.  The macros provied by this crate are also available by
//! enabling the `derive` feature in aforementioned `engcon` crate.

use syn::parse_macro_input;
use syn::DeriveInput;

mod helper;
mod validator;

/// Makes a type validateable, means it implements the [engcon::Validator] trait.
///
/// Core idea is to get the domain contracts from experts, i.e. engineers or
/// other staff, in an easy readable approach for easy interdisciplinary discussions.
///
/// # Example - Distillation Column
///
/// ```
/// use engcon::*;
/// use engcon_macros::Validatable;
/// #[derive(Debug, Clone, Default, Copy, PartialEq, Validatable)]
/// pub struct DistillationColumn {
///     #[validate_value(x >= 3)]
///     pub trays: i32,
///     #[validate_value(x < trays, x >= 1)]
///     pub feed_place: i32,
///     #[validate_value(x > 0.0)]
///     pub reflux_ratio: f32,
///     //#[serde(rename = "d2f")]
///     #[validate_value(x > 0.0, x < 1.0)]
///     pub distiliate_to_feed_ratio: f32,
/// }
/// ```
#[proc_macro_derive(Validatable, attributes(validate_value))]
pub fn validate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    match validator::validator_macro_wrapper(input) {
        Ok(generated_code) => generated_code,
        Err(compile_error) => compile_error,
    }
}
