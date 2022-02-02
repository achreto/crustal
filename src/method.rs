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

//! # Method
//!
//! This module contains definitions for C++ class methods

use std::fmt::{self, Write};

use crate::{Doc, Formatter, MethodParam, Stmt, Type, Visibility};

//
//Default constructor
// Copy constructor
// Move constructor
// Destructor

/// holds a method definition
#[derive(Debug, Clone)]
pub struct Method {
    /// Name of the method
    name: String,

    /// the visibility of the method
    visibility: Visibility,

    /// the method documentation
    doc: Option<Doc>,

    /// the method arguments
    args: Vec<MethodParam>,

    /// the return type of the method
    ret: Type,

    /// whether the method is static
    is_static: bool,

    /// whether this is an inline method
    is_inline: bool,

    /// whether the method is virtual
    is_virtual: bool,

    /// sets the pure
    is_pure: bool,

    /// whether the method is override
    is_override: bool,

    /// sets the method to be const
    is_const: bool,

    /// wheter the definition is inside of the class
    is_inside: bool,

    /// the body of the method, a sequence of statements
    body: Vec<Stmt>,
}

impl Method {
    /// Creates a new method definition
    pub fn new(name: &str, ret: Type) -> Self {
        Self {
            name: String::from(name),
            doc: None,
            visibility: Visibility::Private,
            args: Vec::new(),
            ret,
            is_static: false,
            is_inline: false,
            is_virtual: false,
            is_pure: false,
            is_override: false,
            is_const: false,
            is_inside: false,
            body: Vec::new(),
        }
    }

    /// adds a string to the documentation comment to the variant
    pub fn doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// adds a documetnation comment to the variant
    pub fn add_doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the visibility of the method
    pub fn set_visibility(&mut self, vis: Visibility) -> &mut Self {
        self.visibility = vis;
        self
    }

    /// tests if the method is private
    pub fn is_public(&self) -> bool {
        self.visibility == Visibility::Public
    }

    /// tests if the method is protected
    pub fn is_protected(&self) -> bool {
        self.visibility == Visibility::Protected
    }

    /// tests if the method is private
    pub fn is_private(&self) -> bool {
        self.visibility == Visibility::Private || self.visibility == Visibility::Default
    }

    /// sets the visibility to public
    pub fn public(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Public)
    }

    /// sets the visibility to protected
    pub fn protected(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Protected)
    }

    /// sets the visibility to private
    pub fn private(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Private)
    }

    /// adds an argument to the method
    pub fn add_argument(&mut self, arg: MethodParam) -> &mut Self {
        self.args.push(arg);
        self
    }

    /// sets the method to be overridden
    ///
    /// # Example
    ///
    /// void foo()   -> void foo() override
    pub fn set_override(&mut self, val: bool) -> &mut Self {
        self.is_override = val;
        self
    }

    /// sets the method to override
    pub fn overrid(&mut self) -> &mut Self {
        self.set_const(true)
    }

    /// sets the constant modifier of the method
    ///
    /// # Example
    ///
    /// void foo()   -> void foo() const
    pub fn set_const(&mut self, val: bool) -> &mut Self {
        self.is_const = val;
        self
    }

    /// makes the method to be constant
    pub fn constant(&mut self) -> &mut Self {
        self.set_const(true)
    }

    /// sets the method to be virtual
    ///
    /// # Example
    ///
    /// void foo()   -> virtual void foo() = 0
    pub fn set_virtual(&mut self, val: bool) -> &mut Self {
        if !val {
            self.is_pure = false;
        }
        self.is_virtual = val;
        self
    }

    /// makes the method to be virtual
    pub fn virt(&mut self) -> &mut Self {
        self.set_virtual(true)
    }

    /// sets the method to be pure
    ///
    /// # Example
    ///
    /// void foo()   -> virtual void foo() = 0
    pub fn set_pure(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_virtual = true
        }
        self.is_pure = val;
        self
    }

    /// turns the method into a pure method
    pub fn pure(&mut self) -> &mut Self {
        self.set_pure(true)
    }

    /// sets the method to be static
    ///
    /// # Example
    ///
    /// void foo()   -> static void foo()
    pub fn set_static(&mut self, val: bool) -> &mut Self {
        self.is_static = val;
        self
    }

    /// makes the method to be an static method
    pub fn sstatic(&mut self) -> &mut Self {
        self.set_static(true)
    }

    /// sets the method to be inline
    ///
    /// # Example
    ///
    /// void foo()   -> inline void foo()
    pub fn set_inline(&mut self, val: bool) -> &mut Self {
        self.is_inline = val;
        self
    }

    /// makes the method to be an inline method
    pub fn inline(&mut self) -> &mut Self {
        self.set_inline(true)
    }

    /// sets the definition localtion of the method
    pub fn set_inside_def(&mut self, val: bool) -> &mut Self {
        self.is_inside = val;
        self
    }

    /// this method is defined inside
    pub fn inside_def(&mut self) -> &mut Self {
        self.set_inline(true)
    }

    /// sets the body for the method
    pub fn set_body(&mut self, body: Vec<Stmt>) -> &mut Self {
        if !body.is_empty() {
            self.is_pure = false;
        }
        self.body = body;
        self
    }

    /// pushes a new statement to the method
    pub fn push_stmt(&mut self, stmt: Stmt, decl_only: bool) -> &mut Self {
        self.is_pure = false;
        self.body.push(stmt);
        self
    }

    /// Formats the attribute using the given formatter.
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_static {
            write!(fmt, "static ")?;
        }

        if self.is_inline {
            write!(fmt, "inline ")?;
        }

        if self.is_virtual {
            write!(fmt, "virtual ")?;
        }

        self.ret.fmt(fmt)?;
        write!(fmt, " {}", self.name)?;
        if self.args.is_empty() {
            write!(fmt, "(void)")?;
        } else {
            write!(fmt, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?;
                }
                arg.fmt(fmt)?;
            }
            write!(fmt, ")")?;
        }

        if self.is_const {
            write!(fmt, " const")?;
        }

        if self.is_override {
            write!(fmt, " override")?;
        }

        if self.is_pure {
            return write!(fmt, " = 0;");
        }

        // if we want to have the declaration only, then do that,
        // but only if it's not a inside method or an inline method
        if self.body.is_empty() || (decl_only && !(self.is_inside || self.is_inline)) {
            return writeln!(fmt, ";");
        }

        writeln!(fmt, " {{\n")?;
        fmt.indent(|f| {
            for stmt in &self.body {
                stmt.fmt(f)?;
            }
            Ok(())
        })?;
        write!(fmt, "}}")
    }

    /// formats the method definition
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, false)
    }

    /// formats the method declaration
    pub fn fmt_decl(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, true)
    }

    /// formats the method definition
    pub fn fmt_def(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // inline or inside functions are defined in the declaration
        if self.is_inline || self.is_inside {
            return Ok(());
        }
        self.do_fmt(fmt, false)
    }
}
