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

//! # Formatter
//!
//! This module provides a formatter implementation that emits a string of C code
//! with the right indentation to a buffer.
//!
//! This code is based on the `formatter` module of `codegen-rs`.

// the standard formatter types
use std::fmt::{self, Write};

/// defines the default indentation level
const DEFAULT_INDENT: usize = 4;

/// Formatter for a scope.
#[derive(Debug)]
pub struct Formatter<'a> {
    /// THe desination buffer for the formatter
    dst: &'a mut String,

    /// The current indentation level
    spaces: usize,
}

impl<'a> Formatter<'a> {
    /// Returns a new formatter instance.
    pub fn new(dst: &'a mut String) -> Self {
        Self { dst, spaces: 0 }
    }

    pub fn get_indent(&self) -> usize {
        self.spaces
    }

    /// Wraps the given function in a a C block. { ...}
    pub fn block<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        if !self.is_start_of_line() {
            write!(self, " ")?;
        }

        writeln!(self, "{{")?;
        self.indent(f)?;
        writeln!(self, "}}")?;
        Ok(())
    }

    /// Formats the function with an increased indentation level
    pub fn indent<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        self.spaces += DEFAULT_INDENT;
        let ret = f(self);
        self.spaces -= DEFAULT_INDENT;
        ret
    }

    /// Check if current destination is the start of a new line.
    pub fn is_start_of_line(&self) -> bool {
        self.dst.is_empty() || self.dst.ends_with('\n')
    }

    /// writes spaces into the destination buffer
    fn push_spaces(&mut self) {
        for _ in 0..self.spaces {
            self.dst.push(' ');
        }
    }
}

impl<'a> fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut first = true;
        let mut should_indent = self.is_start_of_line();

        for line in s.lines() {
            if !first {
                self.dst.push('\n');
            }

            first = false;

            let do_indent = should_indent && !line.is_empty() && line.as_bytes()[0] != b'\n';

            if do_indent {
                self.push_spaces();
            }

            // If this loops again, then we just wrote a new line
            should_indent = true;

            self.dst.push_str(line);
        }

        if s.as_bytes().last() == Some(&b'\n') {
            self.dst.push('\n');
        }

        Ok(())
    }
}
