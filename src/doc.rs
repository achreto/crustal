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

//! # Documentation
//!
//! The documentation modules provides a way to add documentation comments to the
//! generated code. Note, to insert normal comments please see the `Comment` module.

use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Documentation.
#[derive(Debug, Clone)]
pub struct Doc {
    /// A vector of documentation lines
    docs: Vec<String>,
}

impl Doc {
    /// creates a new, empty documentation block.
    pub fn new() -> Self {
        Doc { docs: Vec::new() }
    }

    /// creates a new documentation block from a string.
    pub fn with_str(docs: &str) -> Self {
        let mut doc = Doc::new();
        doc.add_text(docs);
        doc
    }

    /// creates a new documentation block from a string.
    pub fn with_string(docs: String) -> Self {
        let mut doc = Doc::new();
        // try to add the string directly here...
        doc.add_text(&docs);
        doc
    }

    /// adds a new line to the documentation block.
    pub fn add_line(&mut self, line: &str) -> &mut Self {
        if line.is_empty() {
            self.docs.push(String::new())
        } else {
            for l in line.lines() {
                self.docs.push(l.to_string());
            }
        }
        self
    }

    /// adds a new textblock as documentation comments, while breaking long lines.
    pub fn add_text(&mut self, text: &str) -> &mut Self {
        let mut res = self;
        let lines = text.lines();
        for l in lines {
            if l.is_empty() || l == "\n" {
                res = res.add_line("");
                continue;
            }
            let mut start = 0;
            let mut end = 0;
            for (offset, c) in l.chars().enumerate() {
                if c == ' ' && (offset - start) > 90 {
                    res = res.add_line(&l[start..=end]);
                    start = end;
                }
                end = offset;
            }

            if start == end {
                res = res.add_line("");
            } else {
                res = res.add_line(&l[start..=end]);
            }
        }
        res
    }

    /// formats the documentation block as a string.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.docs {
            writeln!(fmt, "/// {line}")?;
        }
        Ok(())
    }
}

impl Default for Doc {
    fn default() -> Self {
        Self::new()
    }
}
