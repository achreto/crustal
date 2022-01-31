// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2022 Reto Achermann (The University of British Columbia)
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

//! # Attribute
//!
//! The attribute module provides functionality to express C++ class attributes
//! (data members) with a given type and name.

use std::fmt::{self, Write};

use crate::{Doc, Formatter, Type};

/// Defines a C++ class attribute (data member)
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The name of the attribute
    name: String,

    /// The type of the field
    ty: Type,

    /// the number of bits in the bitfield
    width: Option<u8>,

    /// the value if the field is constant
    value: Option<String>,

    /// the attribute is static (C++)
    is_static: bool,

    /// The documentation comment of the class attribute
    doc: Option<Doc>,
}

impl Attribute {
    /// Creates a new `Attribute` with a given `name` and `type`.
    pub fn new(name: &str, ty: Type) -> Self {
        Attribute {
            name: String::from(name),
            ty,
            width: None,
            value: None,
            is_static: false,
            doc: None,
        }
    }

    /// obtain a reference to the attribute's type
    pub fn get_type_ref(&self) -> &Type {
        &self.ty
    }

    /// adds a string to the documentation comment to the attribute
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// sets the documentation comment of the attribute
    pub fn set_doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the width of the bitfield
    pub fn set_bitfield_width(&mut self, width: u8) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// sets the field to be static
    pub fn set_static(&mut self, val: bool) -> &mut Self {
        self.is_static = val;
        self
    }

    /// sets the field to be constant
    pub fn set_const(&mut self, val: bool) -> &mut Self {
        self.ty.const_value(val);
        self
    }

    /// sets the field to be volatile
    pub fn set_volatile(&mut self, val: bool) -> &mut Self {
        self.ty.volatile_value(val);
        self
    }

    /// Formats the attribute using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        self.ty.fmt(fmt)?;
        writeln!(fmt, " {};", self.name)
    }
}
