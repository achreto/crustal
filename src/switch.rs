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
pub struct Switch {
    /// the conditional of the if-else block
    cond: Expr,
    /// the cases of this switch statement
    cases: Vec<(Expr, Block)>,
    /// the default branch
    default: Option<Block>,
}

impl Switch {
    /// creates a new switch statement with the supplied conditional
    pub fn new(cond: &Expr) -> Self {
        Self {
            cond: cond.clone(),
            cases: Vec::new(),
            default: None,
        }
    }

    /// sets the then branch of the conditional
    pub fn set_default(&mut self, default: Block) -> &mut Self {
        self.default = Some(default);
        self
    }

    /// obtains a mutable reference to the then branch of the conditional
    pub fn new_case(&mut self, label: Expr) -> &mut Block {
        self.cases.push((label, Block::new()));
        if let Some((_, block)) = self.cases.last_mut() {
            block
        } else {
            unreachable!()
        }
    }

    /// obtains a mutable reference to the else branch of the conditional
    pub fn case(&mut self, label: Expr, block: Block) -> &mut Self {
        self.cases.push((label, block));
        self
    }

    /// formats the conditional
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "switch (")?;
        self.cond.fmt(fmt)?;
        writeln!(fmt, ") {{")?;
        for (label, block) in self.cases.iter() {
            writeln!(fmt, "case {}:", label)?;
            fmt.block(|f| block.fmt(f))?;
            writeln!(fmt, "\nbreak;")?;
        }

        if let Some(def) = &self.default {
            writeln!(fmt, "default: ")?;
            fmt.block(|f| def.fmt(f))?;
            writeln!(fmt)?;
        }
        writeln!(fmt, "}}")
    }
}

impl Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{ret}")
    }
}
