// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2022 Reto Achermann
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

//! # Loops
//!
//! This module defines various loop structures such as while, for and dowhile loops.

// std includes
use std::fmt::{self, Display, Write};

use crate::{Block, Expr, Formatter};

#[derive(Debug, Clone)]
pub struct WhileLoop {
    /// the conditional expression of the loop
    cond: Expr,
    /// the body of the loop
    body: Block,
}

impl WhileLoop {
    /// creates a new while loop with the supplied conditional
    pub fn new(cond: &Expr) -> Self {
        Self::with_guard(cond.clone())
    }

    /// creates a new while loop taking ownership of the supplied conditional
    pub fn with_guard(cond: Expr) -> Self {
        WhileLoop { cond, body: Block::new() }
    }

    /// creates a new while loop with the supplied conditional and body
    pub fn with_guard_and_body(cond: Expr, body: Block) -> Self {
        WhileLoop { cond, body }
    }

    /// sets the body block of the while loop
    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    /// obtains a mutable reference to the body block of the loop
    pub fn body(&mut self) -> &mut Block {
        &mut self.body
    }

    /// formats the loop
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "while (")?;
        self.cond.fmt(fmt)?;
        writeln!(fmt, ") ")?;
        if !self.body.is_empty() {
            fmt.block(|f| self.body.fmt(f))?;
            writeln!(fmt)
        } else {
            fmt.indent(|f| writeln!(f, ";"))
        }
    }
}

impl Display for WhileLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{ret}")
    }
}

#[derive(Debug, Clone)]
pub struct DoWhileLoop {
    cond: Expr,
    body: Block,
}

impl DoWhileLoop {
    /// creates a new while loop with the supplied conditional
    pub fn new(cond: &Expr) -> Self {
        Self::with_guard(cond.clone())
    }

    /// creates a new while loop taking ownership of the supplied conditional
    pub fn with_guard(cond: Expr) -> Self {
        DoWhileLoop { cond, body: Block::new() }
    }

    /// creates a new while loop with the supplied conditional and body
    pub fn with_guard_and_body(cond: Expr, body: Block) -> Self {
        DoWhileLoop { cond, body }
    }

    /// sets the body block of the while loop
    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    /// obtains a mutable reference to the body block of the loop
    pub fn body(&mut self) -> &mut Block {
        &mut self.body
    }

    /// formats the loop
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "do ")?;
        fmt.block(|f| self.body.fmt(f))?;
        write!(fmt, " while (")?;
        self.cond.fmt(fmt)?;
        writeln!(fmt, ");")
    }
}

impl Display for DoWhileLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{ret}")
    }
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    init: Option<Expr>,
    cond: Option<Expr>,
    step: Option<Expr>,
    body: Block,
}

impl ForLoop {
    /// creates a new while loop with the supplied conditional
    pub fn new() -> Self {
        ForLoop {
            init: None,
            cond: None,
            step: None,
            body: Block::new(),
        }
    }

    /// creates a new while loop taking ownership of the supplied conditional
    pub fn from_expr(init: &Expr, cond: &Expr, step: &Expr) -> Self {
        Self::with_guard(init.clone(), cond.clone(), step.clone())
    }

    /// creates a new while loop with the supplied conditional and body
    pub fn with_guard(init: Expr, cond: Expr, step: Expr) -> Self {
        ForLoop {
            init: Some(init),
            cond: Some(cond),
            step: Some(step),
            body: Block::new(),
        }
    }

    /// creates a new while loop with the supplied conditional and body
    pub fn with_guard_and_body(init: Expr, cond: Expr, step: Expr, body: Block) -> Self {
        ForLoop {
            init: Some(init),
            cond: Some(cond),
            step: Some(step),
            body,
        }
    }

    /// sets the body block of the while loop
    pub fn set_body(&mut self, body: Block) -> &mut Self {
        self.body = body;
        self
    }

    /// obtains a mutable reference to the body block of the loop
    pub fn body(&mut self) -> &mut Block {
        &mut self.body
    }

    /// formats the loop
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "for (")?;
        if let Some(init) = &self.init {
            init.fmt(fmt)?;
        }
        write!(fmt, "; ")?;
        if let Some(cond) = &self.cond {
            cond.fmt(fmt)?;
        }
        write!(fmt, "; ")?;
        if let Some(step) = &self.step {
            step.fmt(fmt)?;
        }
        writeln!(fmt, ") ")?;
        if self.body.is_empty() {
            fmt.block(|f| self.body.fmt(f))?;
            writeln!(fmt)
        } else {
            fmt.indent(|f| writeln!(f, ";"))
        }
    }
}

impl Default for ForLoop {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ForLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{ret}")
    }
}
