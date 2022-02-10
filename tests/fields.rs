// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2021, 2022 Reto Achermann (The University of British Columbia)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Field Tests
//!
//! This module exercises the field tests

use crustal::*;

#[test]
fn test_fields_basics() {
    let t = Type::new(BaseType::UInt8);
    assert_eq!(t.to_string(), "uint8_t");

    let f = Field::new("my_field", t);
    assert_eq!(f.to_string(), "uint8_t my_field;\n");
}

#[test]
fn test_fields_bitfields() {
    let t = Type::new(BaseType::UInt8);
    assert_eq!(t.to_string(), "uint8_t");

    let mut f = Field::new("my_field", t);
    f.bitfield_width(8);
    assert_eq!(f.to_string(), "uint8_t my_field : 8;\n");
}

#[test]
fn test_fields_docs() {
    let t = Type::new(BaseType::UInt8);
    assert_eq!(t.to_string(), "uint8_t");

    let mut f = Field::new("my_field", t);
    f.push_doc_str("my documentation");
    assert_eq!(f.to_string(), "/// my documentation\nuint8_t my_field;\n");
}
