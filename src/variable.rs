// cgen-rs
//
//
// MIT License
//
// Copyright (c) 2021 Reto Achermann
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

//! # Variables
//!
//! The variables module provides a way to add variable declarations to the
//! generated code.

use std::fmt::{self, Write};

use crate::doc::Doc;
use crate::formatter::Formatter;
use crate::r#type::Type;

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct Variable {
    /// The name of the field/parameter
    name: String,

    /// The type of the field
    ty: Type,

    /// whether or not the variable is static
    is_static: bool,

    /// whether or not the variable is extern
    is_extern: bool,

    /// The documentation comment of the variant
    doc: Option<Doc>,
}

impl Variable {
    /// Creates a new `Variable`
    pub fn new(name: &str, ty: Type) -> Self {
        Variable {
            name: String::from(name),
            ty,
            is_static: false,
            is_extern: false,
            doc: None,
        }
    }

    /// adds a string to the documentation comment to the variant
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// adds a documetnation comment to the variant
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// makes the variable static, and makes it non extern
    pub fn set_static(&mut self) -> &mut Self {
        self.is_static = true;
        self.is_extern = false;
        self
    }

    /// makes the variable static, and makes it non extern
    pub fn set_extern(&mut self) -> &mut Self {
        self.is_static = false;
        self.is_extern = true;
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        if self.is_extern {
            write!(fmt, "extern ")?;
        }
        if self.is_static {
            write!(fmt, "static ")?;
        }
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)
    }
}
