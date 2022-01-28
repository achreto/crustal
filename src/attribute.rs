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

//! # Attribute
//!
//! The field module provides a way to add fields to a struct

use std::fmt::{self, Write};

use crate::doc::Doc;
use crate::formatter::Formatter;
use crate::r#type::{Access, Type};

/// Defines a class attribute member
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The name of the attribute
    name: String,

    /// The type of the field
    ty: Type,

    /// the access modifier
    access: Access,

    /// The documentation comment of the class attribute
    doc: Option<Doc>,
}

impl Attribute {
    /// Creates a new `Attribute`
    pub fn new(name: &str, ty: Type) -> Self {
        Attribute {
            name: String::from(name),
            ty,
            access: Access::Private,
            doc: None,
        }
    }

    /// adds a string to the documentation comment to the attribute
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// adds a documetnation comment to the attribute
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the width of the bitfield
    pub fn set_public(&mut self) -> &mut Self {
        self.access = Access::Public;
        self
    }

    /// sets the width of the bitfield
    pub fn set_protected(&mut self) -> &mut Self {
        self.access = Access::Public;
        self
    }

    /// Formats the attribute using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        self.access.fmt(fmt)?;
        self.ty.fmt(fmt)?;
        writeln!(fmt, " {};", self.name)
    }
}
