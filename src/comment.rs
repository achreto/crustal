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

//! # Comment
//!
//! The documentation modules provides a way to add general comments to the
//! generated code.

use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// defines a comment block
#[derive(Debug, Clone)]
pub struct Comment {
    /// the comment string
    comment: String,
    /// defines whether the comment is a heading
    is_heading: bool,
}

impl Comment {
    /// creates a new comment
    pub fn new(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            is_heading: false,
        }
    }

    /// creates a new heading comment
    pub fn new_heading(comment: &str) -> Self {
        Self {
            comment: comment.to_string(),
            is_heading: false,
        }
    }

    /// pushes the heading separator
    fn push_heading(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.is_heading {
            for _ in 0..(100 - fmt.get_indent()) {
                write!(fmt, "/")?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }

    // formats the comment block
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.push_heading(fmt)?;
        for line in self.comment.lines() {
            writeln!(fmt, "// {}", line)?;
        }
        self.push_heading(fmt)
    }
}
