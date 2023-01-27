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

//! # Macro
//!
//! The macro module provides functionality to define pre-processor macros.

use std::fmt::{self, Write};

use crate::doc::Doc;
use crate::formatter::Formatter;

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct Macro {
    /// The name of the define
    name: String,

    /// the arguments of the macro
    args: Vec<String>,

    /// the value of the define
    value: Option<String>,

    /// The documentation comment of the macro
    doc: Option<Doc>,
}

impl Macro {
    /// Creates a new `Macro`
    pub fn new(name: &str) -> Self {
        Self::with_name(String::from(name))
    }

    /// Creates a new `Macro` with the given name String
    pub fn with_name(name: String) -> Self {
        Macro {
            name,
            args: Vec::new(),
            value: None,
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

    /// adds a new argument to the macro
    pub fn new_arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(String::from(arg));
        self
    }

    /// adds the value to the macro
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value = Some(String::from(value));
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        write!(fmt, "#define {} ", self.name)?;
        if !self.args.is_empty() {
            let args = self.args.join(", ");
            write!(fmt, "({args})")?;
        }

        if let Some(v) = &self.value {
            fmt.indent(|f| {
                for (i, l) in v.lines().enumerate() {
                    if i != 0 {
                        writeln!(f, "\\")?;
                    }
                    write!(f, "{l}")?;
                }
                writeln!(f)?;
                Ok(())
            })
        } else {
            writeln!(fmt)
        }
    }
}
