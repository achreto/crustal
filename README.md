# CRustAL: The C/C++ from Rust Assembly Library

This crate provides a library with builder API for assembling C/C++ code
from Rust.


## LICENSE

[MIT license](LICENSE)


## CONTRIBUTING

Code contributions are welcome. The submitter must use the *sign-off* feature
for all commits confirming that the submitter has all rights to contribute
the code under the [license](LICENSE) without any additional terms or conditions.

See the [AUTHORS](AUTHORS) file for a list of contributors.


## CREDIT

This crate is inspired by [codegen-rs`](https://crates.io/crates/codegen-rs)


## Installation

To use `crustal` clone the repository into the `lib` folder of your Rust project,
or use [crates.io](https://crates.io)


## Usage

You can use `crustal` by adding the following lines to `Cargo.toml` file.


```toml
[dependencies]
crustal
```

Next, create a `Scope` and use the builder API to create elements in the scope.
Lastly, call `Scope::to_string()` to get formatted C code as a string.

```rust
use crustal as CG;

let mut scope = CG::Scope::new();
scope.set_filename("include/my_file.hpp");

scope.push_doc_str("WARNING: This is auto-generated comment\n");
scope.new_include("stdio.h", true);

scope.new_class("MyClass")
        .set_base("StateBase", CG::Visibility::Public)
        .push_attribute(CG::Attribute::new(
        "name",
        CG::Type::new(CG::BaseType::new_int(8)),
    ));

println!("{}", scope.to_string());
```

produces the output

```cpp
/// WARNING: This is auto-generated comment
#include <stdio.h>
class MyClass : public StateBase {

    private:
    uint64_t name;
}
```