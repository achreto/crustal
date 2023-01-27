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

//! # Include
//!
//! The include module provides mechanisms to specify included headers

use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct Include {
    /// The name of the define
    path: String,

    /// whether this is a system include
    is_system: bool,

    /// The documentation comment of the macro
    doc: Option<String>,
}

impl Include {
    /// Creates a new `Include` struct for project headers
    pub fn new(path: &str) -> Self {
        Self::with_string(String::from(path))
    }

    /// creates a new include with a given string
    pub fn with_string(path: String) -> Self {
        Include {
            path,
            is_system: false,
            doc: None,
        }
    }

    /// Creates a new `Include` for system headers
    pub fn new_system(path: &str) -> Self {
        Include {
            path: String::from(path),
            is_system: true,
            doc: None,
        }
    }

    /// adds a string to the documentation comment to the variant
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
        self.doc = Some(String::from(doc));
        self
    }

    /// adds a new argument to the macro
    pub fn system(&mut self, arg: bool) -> &mut Self {
        self.is_system = arg;
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "#include ")?;
        if self.is_system {
            write!(fmt, "<{}>", self.path)?;
        } else {
            write!(fmt, "\"{}\"", self.path)?;
        }

        if let Some(d) = &self.doc {
            writeln!(fmt, "  // {d}")
        } else {
            writeln!(fmt)
        }
    }
}
