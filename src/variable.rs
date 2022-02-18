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

//! # Variables
//!
//! The variables module provides a way to add variable declarations to the
//! generated code, and refer to storage space.

use std::fmt::{self, Display, Write};

use crate::{Doc, Expr, Formatter, Type};

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct Variable {
    /// The name of the field/parameter
    name: String,

    /// The type of the field
    ty: Type,

    /// the value if the attribute is constant
    value: Option<Expr>,

    /// whether or not the variable is static
    is_static: bool,

    /// whether or not the variable is extern
    is_extern: bool,

    /// The documentation comment of the variant
    doc: Option<Doc>,
}

impl Variable {
    /// Creates a new `Variable`
    pub fn new(name: &str, ty: Type) -> Self {
        Variable::with_string(String::from(name), ty)
    }

    /// Creates a new `Variable` and consumes the given string
    pub fn with_string(name: String, ty: Type) -> Self {
        Variable {
            name,
            ty,
            value: None,
            is_static: false,
            is_extern: false,
            doc: None,
        }
    }

    /// creates an expression from the variable
    pub fn to_expr(&self) -> Expr {
        Expr::Variable {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }

    /// Creates a new `Variable` with a given value
    pub fn with_value(name: &str, ty: Type, val: Expr) -> Self {
        Variable::with_string_and_value(String::from(name), ty, val)
    }

    /// Creates a new `Variable` with a given value and consumes the given string
    pub fn with_string_and_value(name: String, ty: Type, val: Expr) -> Self {
        Variable {
            name,
            ty,
            value: Some(val),
            is_static: false,
            is_extern: false,
            doc: None,
        }
    }

    /// obtains the type from the attribute
    pub fn to_type(&self) -> Type {
        self.ty.clone()
    }

    /// returns a reference to the type of the attribute
    pub fn as_type(&self) -> &Type {
        &self.ty
    }

    /// adds a string to the documentation comment to the variant
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
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

    /// changes the static modifier
    pub fn toggle_static(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_extern = false;
        }
        self.is_static = val;
        self
    }

    /// makes the variable static
    pub fn set_static(&mut self) -> &mut Self {
        self.toggle_static(true)
    }

    /// changes the extern modifier
    pub fn toggle_extern(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_static = false;
        }
        self.is_extern = val;
        self
    }

    /// makes the variable static
    pub fn set_extern(&mut self) -> &mut Self {
        self.toggle_extern(true)
    }

    pub fn set_value(&mut self, val: Expr) -> &mut Self {
        self.value = Some(val);
        self
    }

    /// sets the default value of the attribute
    pub fn set_value_raw(&mut self, val: &str) -> &mut Self {
        self.set_value(Expr::Raw(String::from(val)))
    }

    /// the formatting
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        if self.is_extern {
            write!(fmt, "extern ")?;
        }
        if self.is_static {
            write!(fmt, "static ")?;
        }
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;

        if decl_only || self.value.is_none() || self.is_extern {
            writeln!(fmt, ";")
        } else {
            if let Some(v) = &self.value {
                write!(fmt, " = {}", v)?;
            }
            writeln!(fmt, ";")
        }
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }
        if self.is_extern {
            write!(fmt, "extern ")?;
        }
        if self.is_static {
            write!(fmt, "static ")?;
        }
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
