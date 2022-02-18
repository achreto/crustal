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

//! # Function and Method Parameters
//!
//! The parameters module provides a way to add parameter declarations to the
//! generated code.

use std::fmt::{self, Display, Write};

use crate::{Doc, Expr, Formatter, Type};

/// Defines a function parameter
#[derive(Debug, Clone)]
pub struct FunctionParam {
    /// The name of the parameter
    name: String,

    /// The type of the parameter
    ty: Type,

    /// The documentation comment of the parameter
    doc: Option<Doc>,
}

impl FunctionParam {
    /// Creates a new `Param`
    pub fn new(name: &str, ty: Type) -> Self {
        Self::with_string(String::from(name), ty)
    }

    /// Creates a new FunctionParam with the given anme
    pub fn with_string(name: String, ty: Type) -> Self {
        FunctionParam { name, ty, doc: None }
    }

    /// returns the name of the parameter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// returns a reference to the type of the parameter
    pub fn type_ref(&self) -> &Type {
        &self.ty
    }

    /// creates a type reference of the parameter type
    pub fn to_type(&self) -> Type {
        self.ty.clone()
    }

    /// creates an expression from the variable
    pub fn to_expr(&self) -> Expr {
        Expr::Variable {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }

    /// adds a string to the documentation comment to the parameter
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

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)
    }
}

impl Display for FunctionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}

/// Defines an struct field
#[derive(Debug, Clone)]
pub struct MethodParam {
    /// The name of the field/parameter
    name: String,

    /// the default value for the parameter
    default: Option<String>,

    /// The type of the field
    ty: Type,

    /// The documentation comment of the variant
    doc: Option<Doc>,
}

impl MethodParam {
    /// Creates a new `Param`
    pub fn new(name: &str, ty: Type) -> Self {
        MethodParam {
            name: String::from(name),
            default: None,
            ty,
            doc: None,
        }
    }

    /// returns the name of the parameter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// returns a reference to the type of the parameter
    pub fn type_ref(&self) -> &Type {
        &self.ty
    }

    /// creates a type reference of the parameter type
    pub fn to_type(&self) -> Type {
        self.ty.clone()
    }

    /// creates an expression from the variable
    pub fn to_expr(&self) -> Expr {
        Expr::Variable {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }

    /// adds a string to the documentation comment to the method param
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
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

    /// sets the default value of the method parametere
    pub fn set_default_value(&mut self, val: &str) -> &mut Self {
        self.default = Some(String::from(val));
        self
    }

    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if let Some(s) = &self.default {
            if decl_only {
                write!(fmt, " = {}", s)?;
            }
        }
        Ok(())
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, false)
    }
}

impl Display for MethodParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
