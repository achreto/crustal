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

//! # Variant
//!
//! The variant modules provides a way to add enumeration variants to the
//! generated code.

use std::fmt::{self, Write};

use crate::doc::Doc;
use crate::formatter::Formatter;

/// Defines an enumeration variant
#[derive(Debug, Clone)]
pub struct Variant {
    /// The name of the variant
    name: String,

    /// The value of the variant
    value: Option<u64>,

    /// The documentation comment of the variant
    doc: Option<Doc>,
}

impl Variant {
    /// Creates a new `Variant`
    pub fn new(name: &str) -> Self {
        Variant::with_string(String::from(name))
    }

    /// creates a new `Variant` and consumes the given string
    pub fn with_string(name: String) -> Self {
        Variant {
            name,
            value: None,
            doc: None,
        }
    }

    /// Creates a new `Variant` with a given value
    pub fn new_with_value(name: &str, value: u64) -> Self {
        Variant::with_string_and_value(String::from(name), value)
    }

    /// creates a new `Variant` and consumes the given string and value
    pub fn with_string_and_value(name: String, value: u64) -> Self {
        Variant {
            name,
            value: Some(value),
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

    // adds a documetnation comment to the variant
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the current value
    pub fn set_value(&mut self, value: u64) -> &mut Self {
        self.value = Some(value);
        self
    }

    /// obtains the name of the variant
    pub fn name(&self) -> &str {
        &self.name
    }

    /// obtains the current value of the variant
    pub fn value(&self) -> Option<u64> {
        self.value
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        write!(fmt, "{}", self.name)?;
        if let Some(value) = self.value {
            write!(fmt, " = {value}")?;
        }

        Ok(())
    }
}
