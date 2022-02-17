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

//! # Statements
//!
//! The statement module provides mechanisms to express code statements within
//! functions and methods.

use std::fmt::{self, Display, Write};

use crate::{Expr, Formatter, Type};

/// Defines an statement
#[derive(Debug, Clone)]
pub enum Stmt {
    /// represents a variable declaration
    VarDecl {
        name: String,
        ty: Type,
        is_static: bool,
    },
    /// represents a function call
    FnCall(Expr),
    Assign {
        lhs: Expr,
        rhs: Expr,
    },
    IfElse {
        cond: Expr,
        then: Vec<Stmt>,
        other: Vec<Stmt>,
    },
    WhileLoop {
        cond: Expr,
        body: Vec<Stmt>,
    },
    ForLoop {
        init: Expr,
        cond: Expr,
        step: Expr,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Raw(String),
}

impl Stmt {
    /// creates a new variable declaration statement for a
    pub fn localvar(name: &str, ty: Type) -> Self {
        Stmt::VarDecl {
            name: String::from(name),
            ty,
            is_static: false,
        }
    }

    /// creates a new static variable
    pub fn staticvar(name: &str, ty: Type) -> Self {
        Stmt::VarDecl {
            name: String::from(name),
            ty,
            is_static: true,
        }
    }

    /// creates a new function call statement
    pub fn fn_call(expr: Expr) -> Self {
        Stmt::FnCall(expr)
    }

    pub fn retnone() -> Self {
        Stmt::Return(None)
    }

    pub fn retval(expr: &Expr) -> Self {
        Stmt::Return(Some(expr.clone()))
    }

    pub fn ifthen(cond: Expr, then: Vec<Stmt>) -> Self {
        Self::ifthenelse(cond, then, vec![])
    }

    pub fn ifthenelse(cond: Expr, then: Vec<Stmt>, other: Vec<Stmt>) -> Self {
        Stmt::IfElse { cond, then, other }
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::VarDecl {
                name,
                ty,
                is_static,
            } => {
                if *is_static {
                    write!(fmt, "static ")?;
                }
                ty.fmt(fmt)?;
                writeln!(fmt, " {};", name)
            }

            Stmt::FnCall(e) => {
                e.fmt(fmt)?;
                writeln!(fmt, ";")
            }
            Stmt::Assign { lhs, rhs } => {
                lhs.fmt(fmt)?;
                write!(fmt, " = ")?;
                rhs.fmt(fmt)?;
                writeln!(fmt, ";")
            }
            Stmt::IfElse { cond, then, other } => {
                write!(fmt, "if (")?;
                cond.fmt(fmt)?;
                write!(fmt, ")")?;
                fmt.block(|fmt| {
                    for s in then {
                        s.fmt(fmt)?;
                    }
                    Ok(())
                })?;

                if !other.is_empty() {
                    write!(fmt, " else ")?;
                    fmt.block(|fmt| {
                        for s in other {
                            s.fmt(fmt)?;
                        }
                        Ok(())
                    })?;
                }
                writeln!(fmt)
            }
            Stmt::WhileLoop { cond, body } => fmt.block(|fmt| {
                write!(fmt, "while (")?;
                cond.fmt(fmt)?;
                write!(fmt, ")")?;
                if !body.is_empty() {
                    fmt.block(|fmt| {
                        for s in body {
                            s.fmt(fmt)?;
                        }
                        Ok(())
                    })
                } else {
                    writeln!(fmt, ";")
                }
            }),
            Stmt::ForLoop {
                init,
                cond,
                step,
                body,
            } => {
                write!(fmt, "for (")?;
                init.fmt(fmt)?;
                write!(fmt, "; ")?;
                cond.fmt(fmt)?;
                write!(fmt, "; ")?;
                step.fmt(fmt)?;
                write!(fmt, ")")?;
                if !body.is_empty() {
                    fmt.block(|fmt| {
                        for s in body {
                            s.fmt(fmt)?;
                        }
                        Ok(())
                    })
                } else {
                    writeln!(fmt, ";")
                }
            }
            Stmt::Return(Some(val)) => writeln!(fmt, "return {};", val),
            Stmt::Return(None) => writeln!(fmt, "return;"),
            Stmt::Raw(val) => writeln!(fmt, "{};", val),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
