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

//! # Enumerations
//!
//! This module contains the definition of the C enum construct.
//!

use std::fmt;
use std::fmt::Write;

use crate::doc::Doc;
use crate::formatter::Formatter;
use crate::variant::Variant;

/// Defines a C enum.
#[derive(Debug, Clone)]
pub struct Enum {
    /// the name of the enum
    name: String,

    /// the variants of the enum
    variants: Vec<Variant>,

    /// the documentation comment of the enum
    doc: Option<Doc>,
}

impl Enum {
    /// Returns a new `Enum` instance with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            variants: Vec::new(),
            doc: None,
        }
    }

    /// Adds a new documentation to the enum
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a new doc string to the enum
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// creates a new variant with the given name and value
    pub fn new_variant(&mut self, name: &str, value: Option<u64>) -> &mut Variant {
        self.variants.push(Variant::new(name, value));
        self.variants.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    pub fn push_variant(&mut self, item: Variant) -> &mut Self {
        self.variants.push(item);
        self
    }

    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        write!(fmt, "enum {}", self.name)?;
        fmt.block(|fmt| {
            let mut first = true;
            for variant in &self.variants {
                if first {
                    first = false;
                } else {
                    writeln!(fmt, ",")?;
                }
                variant.fmt(fmt)?;
            }

            Ok(())
        })?;
        writeln!(fmt, ";")
    }
}
