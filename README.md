# Readme - EngCon
[![ci](https://github.com/DarthB/engcon/actions/workflows/ci.yml/badge.svg)](https://github.com/DarthB/engcon/actions/workflows/ci.yml)

EngCon (Engineering Contracts) is a set of macros and traits defining contracts often found in
engineering problems and uses the new-type pattern to add a `Validated<T>` datatype ensures that
the specified contracts are hold.

## Motivation

Let's imagine an use-case where an engineer needs to both:

1. Input data that defines the specifications of a distillation column.
2. Simulate and optimize such a distillation column.

In the first case a relaxed type that allows malformed input (at least temporary) is needed.
Thus type should provide methods that 

In the second case we would like to use Rust type system to ensure that only valid input
for a distillation column.

## Including EngCon in your Project

Import `engcon` and `engcon_macros` into your project by adding the following lines to your Cargo.toml.
engcon_macros contains the macros needed to derive the traits in EngCon.

```toml
[dependencies]
engcon = "0.1"
engcon_macros = "0.1"

# You can also access engcon_macros exports directly through strum using the "derive" feature
engcon = { version = "0.1", features = ["derive"] }
```

This pattern is also used by by the well known [strum crate](https://docs.rs/strum/latest/strum/) that has helpful procedural macros
for enumerations.

## Example Usage

```rust
#[derive(Debug, Clone, Default, Copy, PartialEq, Validatable)]
pub struct DistillationColumn {
    #[validate_value(x >= 3)]
    pub trays: i32,

    #[validate_value(x < trays, x >= 1)]
    pub feed_place: i32,

    #[validate_value(x > 0.0)]
    pub reflux_ratio: f32,

    #[validate_value(x > 0.0, x < 1.0)]
    pub distiliate_to_feed_ratio: f32,
}
```

You can run an example based on the distillation column after cloning by typing `cargo run`.

### Is that a reinvented wheel?

Well, actually I started this project to learn about procedural macros. I learnt that the use-case I had in mind
is often not supported. At least not as a first-class citizen. What I mean is the coupling between variables, as the `feed_place < trays` 
contract in the example. I also perfer the attributes to be formal mathmatical, i.e. with comparsions and boolean logic. 
As I believe that is benefical from an engineers perspective. 

All that said, I found the following crates that may serve your use-case better:

- [Contract](https://docs.rs/contracts/latest/contracts/) - Allows the definition of preconditions, invariants and post-conditions supporting desing by contract.
- [Validator](https://github.com/Keats/validator) - A strong input validator useful for web input, supports e-mail and credit card validators.
- [prae](https://docs.rs/prae/latest/prae/index.html) - A declarative macro that defines types that require validation. It uses closures for validation and small adjustments.

## Roadmap

I have some ideas how to continue, but I also want to get inital feedback.

- Use a composite error type to collect all invalid contracts
- Enhance the automatically generated code
- Overthink the grammar, use `&&` and `||` instead of `,` to put multiple rules in a contract.

## Contributors

The following contributors have either helped to start this project, have contributed code, are actively maintaining it (including documentation), or in other ways being awesome contributors to this project. We'd like to take a moment to recognize them.

[<img src="https://github.com/DarthB.png?size=72" alt="DarthB" width="72">](https://github.com/DarthB)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## License

EngCon is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
