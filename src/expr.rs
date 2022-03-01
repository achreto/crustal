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

use crate::{Attribute, BaseType, Formatter, FunctionParam, MethodParam, Type};

/// Defines an statement
#[derive(Debug, Clone)]
pub enum Expr {
    /// represents a variable with a given type
    Variable {
        name: String,
        ty: Type,
    },
    /// represents a constant in the expressions, e.g., 0, '1', "asdf"
    ConstNum(u64),
    ConstString(String),
    ConstBool(bool),
    /// represents a function call
    FnCall {
        name: String,
        args: Vec<Expr>,
    },
    /// represents a method call
    MethodCall {
        var: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        is_ptr: bool,
    },
    /// represents the dereference operator `*(Expr)`
    Deref(Box<Expr>),
    /// represents the address of operationr: `&(Expr)`
    AddrOf(Box<Expr>),
    /// accesses the field
    FieldAccess {
        var: Box<Expr>,
        field: String,
        is_ptr: bool,
    },
    /// represents a binary opreation: `a + b`
    BinOp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: String,
    },
    /// represents an uniary operator: `!(expr)`
    UnOp {
        expr: Box<Expr>,
        op: String,
    },
    /// represents a raw expression token
    Raw(String),
}

impl Expr {
    pub fn new_str(s: &str) -> Self {
        Expr::ConstString(s.to_string())
    }

    pub fn new_num(n: u64) -> Self {
        Expr::ConstNum(n)
    }

    pub fn new_var(name: &str, ty: Type) -> Self {
        Expr::Variable {
            name: name.to_string(),
            ty,
        }
    }

    pub fn btrue() -> Self {
        Expr::ConstBool(true)
    }

    pub fn bfalse() -> Self {
        Expr::ConstBool(false)
    }

    pub fn uop(op: &str, expr: Expr) -> Self {
        Expr::UnOp {
            expr: Box::new(expr),
            op: op.to_string(),
        }
    }

    pub fn not(expr: Expr) -> Self {
        Expr::uop("!", expr)
    }

    pub fn binop(lhs: Expr, op: &str, rhs: Expr) -> Self {
        Expr::BinOp {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op: op.to_string(),
        }
    }

    pub fn from_fn_param(p: &FunctionParam) -> Self {
        p.to_expr()
    }

    pub fn from_method_param(p: &MethodParam) -> Self {
        p.to_expr()
    }

    pub fn from_attribuge(p: &Attribute) -> Self {
        p.to_expr()
    }

    pub fn this() -> Self {
        Expr::Variable {
            name: "this".to_string(),
            ty: Type::to_ptr(&Type::new(BaseType::Class(String::from("auto")))),
        }
    }

    pub fn addr_of(var: &Expr) -> Self {
        Expr::AddrOf(Box::new(var.clone()))
    }

    pub fn deref(var: &Expr) -> Self {
        Expr::Deref(Box::new(var.clone()))
    }

    pub fn field_access(var: &Expr, field: &str) -> Self {
        Expr::FieldAccess {
            var: Box::new(var.clone()),
            field: field.to_string(),
            is_ptr: false,
        }
    }

    /// TODO: add type information here!
    pub fn method_call(var: &Expr, method: &str, args: Vec<Expr>) -> Self {
        Expr::MethodCall {
            var: Box::new(var.clone()),
            method: method.to_string(),
            args,
            is_ptr: false,
        }
    }

    pub fn fn_call(name: &str, args: Vec<Expr>) -> Self {
        Expr::FnCall {
            name: String::from(name),
            args,
        }
    }

    pub fn set_ptr(&mut self) {
        if let Expr::MethodCall { is_ptr, .. } = self {
            *is_ptr = true;
        }
        if let Expr::FieldAccess { is_ptr, .. } = self {
            *is_ptr = true;
        }
    }

    pub fn is_ptr(&self) -> bool {
        match self {
            Expr::Variable { ty, .. } => ty.is_ptr(),
            Expr::Deref(_e) => {
                panic!("handle me!");
            }
            Expr::AddrOf(_) => true,
            Expr::Raw(_) => true,
            Expr::MethodCall { is_ptr, .. } => *is_ptr,
            Expr::FieldAccess { is_ptr, .. } => *is_ptr,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Expr::Variable { ty, .. } => ty.is_struct(),
            Expr::Raw(_) => true,
            _ => false,
        }
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Variable { name, .. } => write!(fmt, "{}", name),
            Expr::ConstString(x) => write!(fmt, "\"{}\"", x),
            Expr::ConstNum(x) => write!(fmt, "{}", x),
            Expr::ConstBool(true) => write!(fmt, "true"),
            Expr::ConstBool(false) => write!(fmt, "false"),
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
            Expr::FieldAccess { var, field, .. } => {
                var.as_ref().fmt(fmt)?;
                if var.is_ptr() {
                    write!(fmt, "->{}", field)
                } else {
                    write!(fmt, ".{}", field)
                }
            }
            Expr::MethodCall { var, method, args, .. } => {
                var.as_ref().fmt(fmt)?;
                if var.is_ptr() {
                    write!(fmt, "->{}(", method)?;
                } else {
                    write!(fmt, ".{}(", method)?;
                }
                for (i, v) in args.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    v.fmt(fmt)?;
                }
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
