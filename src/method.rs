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
//! This module contains definitions for C++ class methods. Note that this is
//! for ordinary methods only, not constructors or destructors.

use std::fmt::{self, Write};

use crate::{Block, Doc, Formatter, MethodParam, Type, Visibility};

/// holds a method definition
#[derive(Debug, Clone)]
pub struct Method {
    /// Name of the method
    name: String,

    /// the visibility of the method
    visibility: Visibility,

    /// the method documentation
    doc: Option<Doc>,

    /// the method parameters
    params: Vec<MethodParam>,

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
    body: Block,
}

impl Method {
    /// Creates a new method definition
    pub fn new(name: &str, ret: Type) -> Self {
        Self::with_string(String::from(name), ret)
    }

    pub fn with_string(name: String, ret: Type) -> Self {
        Self {
            name,
            doc: None,
            visibility: Visibility::Private,
            params: Vec::new(),
            ret,
            is_static: false,
            is_inline: false,
            is_virtual: false,
            is_pure: false,
            is_override: false,
            is_const: false,
            is_inside: false,
            body: Block::new(),
        }
    }

    /// returns the name of the method
    pub fn name(&self) -> &str {
        &self.name
    }

    /// obtains the type for this function
    pub fn to_type(&self) -> Type {
        panic!("needs to implement a corresponding type.")
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
    pub fn set_public(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Public)
    }

    /// sets the visibility to protected
    pub fn set_protected(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Protected)
    }

    /// sets the visibility to private
    pub fn set_private(&mut self) -> &mut Self {
        self.set_visibility(Visibility::Private)
    }

    /// adds an argument to the method
    pub fn push_param(&mut self, arg: MethodParam) -> &mut Self {
        self.params.push(arg);
        self
    }

    /// creates a new param and adds it to the method
    pub fn new_param(&mut self, name: &str, ty: Type) -> &mut MethodParam {
        self.push_param(MethodParam::new(name, ty));
        self.params.last_mut().unwrap()
    }

    /// obtains a reference to the param with the given name
    pub fn param_by_name(&self, name: &str) -> Option<&MethodParam> {
        self.params.iter().find(|f| f.name() == name)
    }

    /// obtains a mutable reference to the param with the given name
    pub fn param_by_name_mut(&mut self, name: &str) -> Option<&mut MethodParam> {
        self.params.iter_mut().find(|f| f.name() == name)
    }

    /// obtains a reference to the param with the given index (starting at 0)
    pub fn param_by_idx(&self, idx: usize) -> Option<&MethodParam> {
        self.params.get(idx)
    }

    /// obtains a mutable reference to the param with the given index mut
    pub fn param_by_idx_mut(&mut self, idx: usize) -> Option<&mut MethodParam> {
        self.params.get_mut(idx)
    }

    /// sets the method to be overridden
    ///
    /// # Example
    ///
    /// void foo()   -> void foo() override
    pub fn toggle_override(&mut self, val: bool) -> &mut Self {
        self.is_override = val;
        self
    }

    /// sets the method to override
    pub fn set_override(&mut self) -> &mut Self {
        self.toggle_override(true)
    }

    /// sets the constant modifier of the method
    ///
    /// # Example
    ///
    /// void foo()   -> void foo() const
    pub fn toggle_const(&mut self, val: bool) -> &mut Self {
        self.is_const = val;
        self
    }

    /// makes the method to be constant
    pub fn set_const(&mut self) -> &mut Self {
        self.toggle_const(true)
    }

    /// sets the method to be virtual
    ///
    /// # Example
    ///
    /// void foo()   -> virtual void foo() = 0
    pub fn toggle_virtual(&mut self, val: bool) -> &mut Self {
        if !val {
            self.is_pure = false;
        }
        self.is_virtual = val;
        self
    }

    /// makes the method to be virtual
    pub fn set_virtual(&mut self) -> &mut Self {
        self.toggle_virtual(true)
    }

    /// sets the method to be pure
    ///
    /// # Example
    ///
    /// void foo()   -> virtual void foo() = 0
    pub fn toggle_pure(&mut self, val: bool) -> &mut Self {
        if val {
            self.body.clear();
            self.is_virtual = true
        }
        self.is_pure = val;
        self
    }

    /// turns the method into a pure method
    pub fn set_pure(&mut self) -> &mut Self {
        self.toggle_pure(true)
    }

    /// sets the method to be static
    ///
    /// # Example
    ///
    /// void foo()   -> static void foo()
    pub fn toggle_static(&mut self, val: bool) -> &mut Self {
        self.is_static = val;
        self
    }

    /// makes the method to be an static method
    pub fn set_static(&mut self) -> &mut Self {
        self.toggle_static(true)
    }

    /// sets the method to be inline
    ///
    /// # Example
    ///
    /// void foo()   -> inline void foo()
    pub fn toggle_inline(&mut self, val: bool) -> &mut Self {
        self.is_inline = val;
        self
    }

    /// makes the method to be an inline method
    pub fn set_inline(&mut self) -> &mut Self {
        self.toggle_inline(true)
    }

    /// sets the definition localtion of the method
    pub fn toggle_inside_def(&mut self, val: bool) -> &mut Self {
        self.is_inside = val;
        self
    }

    /// this method is defined inside
    pub fn set_inside_def(&mut self) -> &mut Self {
        self.toggle_inside_def(true)
    }

    /// sets the body for the method
    pub fn set_body(&mut self, body: Block) -> &mut Self {
        if !body.is_empty() {
            self.is_pure = false;
        }
        self.body = body;
        self
    }

    /// obtains a mutable reference to the body
    pub fn body(&mut self) -> &mut Block {
        &mut self.body
    }

    /// Formats the attribute using the given formatter.
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if !self.body.is_empty() | self.doc.is_some() {
            writeln!(fmt)?;
        }

        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_static && decl_only {
            write!(fmt, "static ")?;
        }

        if self.is_inline {
            write!(fmt, "inline ")?;
        }

        if self.is_virtual && decl_only {
            write!(fmt, "virtual ")?;
        }

        self.ret.fmt(fmt)?;
        if decl_only {
            write!(fmt, " {}", self.name)?;
        } else {
            fmt.write_scoped_name(self.name.as_str())?;
        }
        if self.params.is_empty() {
            write!(fmt, "(void)")?;
        } else {
            write!(fmt, "(")?;
            for (i, arg) in self.params.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?;
                }
                arg.fmt(fmt)?;
            }
            write!(fmt, ")")?;
        }

        if self.is_const && decl_only {
            write!(fmt, " const")?;
        }

        if self.is_override && decl_only {
            write!(fmt, " override")?;
        }

        if self.body.is_empty() && self.is_pure && decl_only {
            return write!(fmt, " = 0;");
        }

        // if we want to have the declaration only, then do that,
        // but only if it's not a inside method or an inline method
        if self.body.is_empty() || (decl_only && !(self.is_inside || self.is_inline)) {
            return writeln!(fmt, ";");
        }

        fmt.block(|f| self.body.fmt(f))?;
        writeln!(fmt)
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
