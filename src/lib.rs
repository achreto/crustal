// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2021, 2022 Reto Achermann
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

//! # Cgen Rust Library
//!
//! The Cgen Rust library provides a builder API for generating C code.

mod attribute;
mod block;
mod class;
mod comment;
mod constructor;
mod doc;
mod expr;
mod field;
mod formatter;
mod function;
mod ifdef;
mod ifelse;
mod include;
mod loops;
mod method;
mod param;
mod scope;
mod switch;
mod union;
mod variable;
mod variant;

mod r#enum;
mod r#macro;
mod r#struct;
mod r#type;

pub use attribute::Attribute;
pub use block::Block;
pub use class::Class;
pub use comment::Comment;
pub use constructor::{Constructor, Destructor};
pub use doc::Doc;
pub use expr::Expr;
pub use field::Field;
use formatter::Formatter;
pub use function::Function;
pub use ifdef::IfDef;
pub use ifelse::IfElse;
pub use include::Include;
pub use loops::{DoWhileLoop, ForLoop, WhileLoop};
pub use method::Method;
pub use param::{FunctionParam, MethodParam};
pub use r#macro::Macro;
pub use scope::Scope;
pub use switch::Switch;
pub use union::Union;
pub use variable::Variable;
pub use variant::Variant;

pub use r#enum::Enum;
pub use r#struct::Struct;
pub use r#type::{BaseType, Type, Visibility};
