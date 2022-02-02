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

//! # Expressions
//!
//! The statement module provides mechanisms to express code statements.

use std::fmt::{self, Display, Write};

use crate::{Formatter, Type};

/// Defines an statement
#[derive(Debug, Clone)]
pub enum Expr {
    /// represents a variable with a given type
    Variable { name: String, ty: Type },
    /// represents a constant in the expressions, e.g., 0, '1', "asdf"
    Const(String),
    /// represents a function call
    FnCall { name: String, args: Vec<Expr> },
    /// represents the dereference operator `*(Expr)`
    Deref(Box<Expr>),
    /// represents the address of operationr: `&(Expr)`
    AddrOf(Box<Expr>),
    /// represents a binary opreation: `a + b`
    BinOp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: String,
    },
    /// represents an uniary operator: `!(expr)`
    UnOp { expr: Box<Expr>, op: String },
    /// represents a raw expression token
    Raw(String),
}

impl Expr {
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Variable { name, .. } => write!(fmt, "{}", name),
            Expr::Const(x) => write!(fmt, "{}", x),
            Expr::FnCall { name, args } => {
                write!(fmt, "{}(", name)?;
                for (i, v) in args.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    v.fmt(fmt)?;
                }
                write!(fmt, ")")
            }
            Expr::Deref(e) => {
                write!(fmt, "*(")?;
                e.as_ref().fmt(fmt)?;
                write!(fmt, ")")
            }
            Expr::AddrOf(e) => {
                write!(fmt, "&(")?;
                e.as_ref().fmt(fmt)?;
                write!(fmt, ")")
            }
            Expr::BinOp { lhs, rhs, op } => {
                write!(fmt, "(")?;
                lhs.as_ref().fmt(fmt)?;
                write!(fmt, " {} ", op)?;
                rhs.as_ref().fmt(fmt)?;
                write!(fmt, ")")
            }
            Expr::UnOp { expr, op } => {
                write!(fmt, "{}(", op)?;
                expr.as_ref().fmt(fmt)?;
                write!(fmt, ")")
            }
            Expr::Raw(s) => write!(fmt, "{}", s),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
