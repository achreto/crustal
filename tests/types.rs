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

//! # Type Tests
//!
//! This module exercises the type tests

use crustal::*;

#[test]
fn types_base_types() {
    let t = Type::new(BaseType::Void);
    assert_eq!(t.to_string(), "void");

    let t = Type::new(BaseType::Double);
    assert_eq!(t.to_string(), "double");

    let t = Type::new(BaseType::Float);
    assert_eq!(t.to_string(), "float");

    let t = Type::new(BaseType::Char);
    assert_eq!(t.to_string(), "char");

    let t = Type::new(BaseType::UInt8);
    assert_eq!(t.to_string(), "uint8_t");

    let t = Type::new(BaseType::UInt16);
    assert_eq!(t.to_string(), "uint16_t");

    let t = Type::new(BaseType::UInt32);
    assert_eq!(t.to_string(), "uint32_t");

    let t = Type::new(BaseType::UInt64);
    assert_eq!(t.to_string(), "uint64_t");

    let t = Type::new(BaseType::Int8);
    assert_eq!(t.to_string(), "int8_t");

    let t = Type::new(BaseType::Int16);
    assert_eq!(t.to_string(), "int16_t");

    let t = Type::new(BaseType::Int32);
    assert_eq!(t.to_string(), "int32_t");

    let t = Type::new(BaseType::Int64);
    assert_eq!(t.to_string(), "int64_t");

    let t = Type::new(BaseType::Bool);
    assert_eq!(t.to_string(), "bool");

    let t = Type::new(BaseType::UIntPtr);
    assert_eq!(t.to_string(), "uintptr_t");

    let t = Type::new(BaseType::Bool);
    assert_eq!(t.to_string(), "bool");

    let t = Type::new(BaseType::Enum(String::from("MyEnum")));
    assert_eq!(t.to_string(), "enum MyEnum");

    let t = Type::new(BaseType::Struct(String::from("MyStruct")));
    assert_eq!(t.to_string(), "struct MyStruct");

    let t = Type::new(BaseType::Union(String::from("MyUnion")));
    assert_eq!(t.to_string(), "union MyUnion");

    let t = Type::new(BaseType::Class(String::from("MyClass")));
    assert_eq!(t.to_string(), "MyClass");

    let t = Type::new(BaseType::TemplateClass(
        String::from("MyClass"),
        vec![String::from("MyOtherClass")],
    ));
    assert_eq!(t.to_string(), "MyClass<MyOtherClass>");

    let t = Type::new(BaseType::TypeDef(String::from("mytype_t")));
    assert_eq!(t.to_string(), "mytype_t");
}

#[test]
fn types_base_modifiers() {
    let mut t = Type::new(BaseType::Int32);
    t.set_value_volatile();
    assert_eq!(t.to_string(), "volatile int32_t");

    let mut t = Type::new(BaseType::Int32);
    t.set_value_const();
    assert_eq!(t.to_string(), "const int32_t");

    let mut t = Type::new(BaseType::Int32);
    t.set_value_const().set_value_volatile();
    assert_eq!(t.to_string(), "volatile int32_t");

    let mut t = Type::new(BaseType::Int32);
    t.set_value_volatile().set_value_const();
    assert_eq!(t.to_string(), "const int32_t");
}

#[test]
fn types_modifiers() {
    let mut t = Type::new(BaseType::Int32);
    t.set_value_volatile().pointer();
    assert_eq!(t.to_string(), "volatile int32_t *");

    let mut t = Type::new(BaseType::Int32);
    t.set_value_const().pointer().constant();
    assert_eq!(t.to_string(), "const int32_t * const");

    let mut t = Type::new(BaseType::Int32);
    t.set_value_const().pointer().constant().pointer().pointer().constant();
    assert_eq!(t.to_string(), "const int32_t * const * * const");
}

#[test]
fn types_modifiers_deref() {
    let mut t = Type::new(BaseType::Int32);

    let t1 = t.to_deref();
    assert!(t1.is_none());

    t.set_value_const().pointer().constant().pointer().pointer().constant();
    assert_eq!(t.to_string(), "const int32_t * const * * const");

    let t2 = t.to_deref();
    assert!(!t2.is_none());
    assert_eq!(t2.unwrap().to_string(), "const int32_t * const *");
}
