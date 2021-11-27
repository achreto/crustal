# Cgen-rs

This crate provides a builder API for building C code.

## CREDIT

This crate is inspired by `codegen-rs`.

## LICENSE

[MIT license](LICENSE)

## CONTRIBUTING

Code contributions are welcome. The submitter must use the *sign-off* feature
for all commits confirming that the submitter has all rights to contribute
the code under the [license](LICENSE) without any additional terms aor conditions.

## Installation

To use `cgen-rs` clone the repository into the `lib` folder of your Rust project
until crates.io integration is completed.

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
use cgen_rs::Scope;

let mut scope = Scope::new();

scope.new_struct("Foo")
    .derive("Debug")
    .field("one", "size_t")
    .field("two", "char *");

println!("{}", scope.to_string());
```

## Release

 When releasing to crates.io:
 - Update html_root_url.
 - Update CHANGELOG.md.
 - Update doc URL.
 - Create "vX.Y.Z" git tag.