// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2022 Reto Achermann (The University of British Columbia)
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

//! # Attribute
//!
//! The attribute module provides functionality to express C++ class attributes
//! (data members) with a given type and name. For C struct fields, see the
//! [Field] module.
//!
//! ## Example
//!
//! ```cpp
//!     public:
//!     static bool foo;
//! ```
//!
//!

use std::fmt::{self, Display, Write};

use crate::{Doc, Expr, Formatter, Type, Visibility};

/// Defines a C++ class attribute (data member)
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The name of the attribute
    name: String,

    /// the visibility of the function
    visibility: Visibility,

    /// The type of the attribute
    ty: Type,

    /// the number of bits in the bitattribute
    width: Option<u8>,

    /// the value if the attribute is constant
    value: Option<Expr>,

    /// the attribute is static
    is_static: bool,

    /// The documentation comment of the class attribute
    doc: Option<Doc>,
}

impl Attribute {
    /// Creates a new `Attribute` with a given `name` and `type`. The attribute is
    /// private by default.
    pub fn new(name: &str, ty: Type) -> Self {
        Attribute::with_string(String::from(name), ty)
    }

    /// Creates a new `Attribute` and consumes the given name
    pub fn with_string(name: String, ty: Type) -> Self {
        Attribute {
            name,
            ty,
            visibility: Visibility::Default,
            width: None,
            value: None,
            is_static: false,
            doc: None,
        }
    }

    /// obtains a string reference to the name of the attribute
    pub fn name(&self) -> &str {
        &self.name
    }

    /// gets a copy of the type information for this attribute
    pub fn to_type(&self) -> Type {
        self.ty.clone()
    }

    /// returns a reference to the type information for this attribute
    pub fn as_type(&self) -> &Type {
        &self.ty
    }

    /// creates an expression from the attribute; no using the `this->` operator
    pub fn to_expr(&self) -> Expr {
        Expr::Variable {
            name: self.name.clone(),
            ty: self.ty.clone(),
        }
    }

    /// returns the visibility of the attribute
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// tests if the attribute is public
    pub fn is_public(&self) -> bool {
        self.visibility == Visibility::Public
    }

    /// tests if the attribute is protected
    pub fn is_protected(&self) -> bool {
        self.visibility == Visibility::Protected
    }

    /// tests if the attribute is private, takes default as private
    pub fn is_private(&self) -> bool {
        self.visibility == Visibility::Private || self.visibility == Visibility::Default
    }

    /// sets the visibility of the attribute
    pub fn set_visibility(&mut self, vis: Visibility) -> &mut Self {
        self.visibility = vis;
        self
    }

    /// makes the attribute public
    pub fn set_public(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Public)
    }

    /// makes the attribute protected
    pub fn set_protected(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Protected)
    }

    /// makes the attribute private
    pub fn set_private(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Private)
    }

    /// Pushes a new string to the the documentation comment of the attribute
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// replaces the documentation comment of the attribute
    pub fn set_doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the width of the bitattribute, if the type is an integer
    ///
    /// Note: only doesn't check the integer width
    pub fn set_bitfield_width(&mut self, width: u8) -> &mut Self {
        // only allow this for integer types
        if self.ty.is_integer() {
            self.width = Some(width);
        }
        self
    }

    /// tests whether this is a bitfield attribute
    pub fn is_bitfield(&self) -> bool {
        self.width.is_some()
    }

    /// sets the static property of the attribute
    pub fn toggle_static(&mut self, val: bool) -> &mut Self {
        self.is_static = val;
        self
    }

    /// makes the attribute static
    pub fn set_static(&mut self) -> &mut Self {
        self.toggle_static(true)
    }

    /// tests whether the attribute is static
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// sets the initializer value for the attribute
    pub fn set_value(&mut self, val: Expr) -> &mut Self {
        self.value = Some(val);
        self
    }

    /// obtains a reference to the initializer value for the attribute
    pub fn value(&self) -> Option<&Expr> {
        self.value.as_ref()
    }

    /// formats the declaration of the attribute
    pub fn fmt_decl(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if let Some(w) = self.width {
            write!(fmt, " : {}", w)?;
        }

        writeln!(fmt, ";")
    }

    /// formats the definition of the attribute
    pub fn fmt_def(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // only need to format the attribute if it's static as we need space for it
        if !self.is_static {
            return Ok(());
        }

        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if let Some(w) = self.width {
            write!(fmt, " : {}", w)?;
        }

        if let Some(v) = &self.value {
            write!(fmt, " = {}", v)?;
        }

        writeln!(fmt, ";")
    }

    /// formats the attribute declaration or definition into the provided formatter
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        self.ty.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if let Some(w) = self.width {
            write!(fmt, " : {}", w)?;
        }

        if let Some(v) = &self.value {
            if !decl_only {
                write!(fmt, " = {}", v)?;
            }
        }
        writeln!(fmt, ";")
    }

    /// Formats the attribute using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, false)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
