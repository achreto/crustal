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
pub struct IfElse {
    /// the conditional of the if-else block
    cond: Expr,
    /// the then branch of the if-else block
    then: Block,
    /// the else branch of the if-else block
    other: Block,
}

impl IfElse {
    /// creates a new if-else block with the supplied conditional
    pub fn new(cond: &Expr) -> Self {
        Self::with_expr(cond.clone())
    }

    /// creates a new if-else block with the supplied expression
    pub fn with_expr(cond: Expr) -> Self {
        IfElse {
            cond,
            then: Block::new(),
            other: Block::new(),
        }
    }

    /// creates a new guard with the supplied conditional and body
    pub fn with_block(cond: Expr, then: Block) -> Self {
        IfElse {
            cond,
            then,
            other: Block::new(),
        }
    }

    /// sets the then branch of the conditional
    pub fn set_then(&mut self, then: Block) -> &mut Self {
        self.then = then;
        self
    }

    /// sets the else branch of the conditional
    pub fn set_other(&mut self, other: Block) -> &mut Self {
        self.other = other;
        self
    }

    /// obtains a mutable reference to the then branch of the conditional
    pub fn then_branch(&mut self) -> &mut Block {
        &mut self.then
    }

    /// obtains a mutable reference to the else branch of the conditional
    pub fn other_branch(&mut self) -> &mut Block {
        &mut self.other
    }

    /// formats the conditional
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "if (")?;
        self.cond.fmt(fmt)?;
        writeln!(fmt, ") ")?;
        fmt.block(|f| self.then.fmt(f))?;
        if !self.other.is_empty() {
            write!(fmt, " else ")?;
            fmt.block(|f| self.other.fmt(f))?;
        }
        writeln!(fmt)
    }
}

impl Display for IfElse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
