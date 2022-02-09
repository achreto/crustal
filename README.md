# Cgen-rs

This crate provides a builder API for building C code.

## CREDIT

This crate is inspired by `codegen-rs`.

## LICENSE

[MIT license](LICENSE)

## CONTRIBUTING

Code contributions are welcome. The submitter must use the *sign-off* feature
for all commits confirming that the submitter has all rights to contribute
the code under the [license](LICENSE) without any additional terms or conditions.

## Installation

To use `cgen-rs` clone the repository into the `lib` folder of your Rust project,
or use [crates.io](https://crates.io)

## Usage

You can use `cgen-rs` by adding the following lines to `Cargo.toml` file.

To use `codegen-rs`, first add this to your `Cargo.toml`:

```toml
[dependencies]
cgen-rs = { path = "lib/cgen-rs" }
```

Next, create a `Scope` and use the builder API to create elements in the scope.
Lastly, call `Scope::to_string()` to get formatted C code as a string.

```rust
use cgen_rs as CG;

let mut scope = CG::Scope::new();
scope.set_filename("include/my_file.hpp");

scope.push_doc_str("WARNING: This is auto-generated comment\n");
scope.new_include("stdio.h", true);

scope.new_class("MyClass")
    .set_base("StateBase", CG::Visibility::Public)
    .push_attribute(Attribute::new("name", Type::new_int(8)));

println!("{}", scope.to_string());
```
