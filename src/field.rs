// C/C++ Code Generator For Rust
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

//! # Struct Field
//!
//! The field module provides functionality to express struct fields as in C.
//! Note, for data members in classes see [Attribute]

use std::fmt::{self, Display, Write};

use crate::{Doc, Formatter, Type};

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct Field {
    /// The name of the field/parameter
    name: String,

    /// The type of the field
    ty: Type,

    /// the number of bits in the bitfield
    width: Option<u8>,

    /// The documentation comment of the variant
    doc: Option<Doc>,
}

impl Field {
    /// Creates a new `Field`
    pub fn new(name: &str, ty: Type) -> Self {
        Field {
            name: String::from(name),
            ty,
            width: None,
            doc: None,
        }
    }

    /// obtains the name of the field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// obtains the type from the field
    pub fn as_type(&self) -> Type {
        self.ty.clone()
    }

    /// returns a reference to the type of the field
    pub fn as_type_ref(&self) -> &Type {
        &self.ty
    }

    /// adds a string to the documentation comment to the variant
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// adds a documetnation comment to the variant
    pub fn set_doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the bitfield width
    pub fn bitfield_width(&mut self, width: u8) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if let Some(w) = self.width {
            write!(fmt, " : {}", w)?;
        }
        writeln!(fmt, ";")
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
