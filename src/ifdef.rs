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

use std::fmt::{self, Write};

use crate::{Formatter, Scope};

/// defines a comment block
#[derive(Debug, Clone)]
pub struct IfDef {
    /// the symbol to be defined
    sym: String,

    /// the then branch
    then: Scope,

    /// the other branch
    other: Option<Scope>,
}

impl IfDef {
    /// creates a new comment
    pub fn new(sym: &str) -> Self {
        Self {
            sym: sym.to_string(),
            then: Scope::new(),
            other: None,
        }
    }

    /// obtains the scope to the then block
    pub fn then_scope(&mut self) -> &mut Scope {
        &mut self.then
    }

    /// obtains the scope to the other block
    pub fn other_scope(&mut self) -> &mut Scope {
        &mut self.then
    }

    // formats the ifdef block
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "#ifdef {}", self.sym)?;
        self.then.fmt(fmt)?;
        if let Some(b) = &self.other {
            b.fmt(fmt)?;
        }
        writeln!(fmt, "#endif // {}", self.sym)
    }
}
