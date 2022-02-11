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

//! # Functions
//!
//! This module contains definitions for C-style function definitions and declarations
//! For C++ class methods, see [Method]

use std::fmt::{self, Display, Write};

use crate::{Doc, Formatter, FunctionParam, Stmt, Type};

/// defines a C function
#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function
    name: String,

    /// the function documentation
    doc: Option<Doc>,

    /// the function arguments
    params: Vec<FunctionParam>,

    /// the return type of the function
    ret: Type,

    /// attributes of the function
    attributes: Vec<String>,

    /// whether the function is static
    is_static: bool,

    /// whether this is an inline function
    is_inline: bool,

    /// whether the function is extern
    is_extern: bool,

    /// the body of the function, a sequence of statements
    body: Vec<Stmt>,
}

impl Function {
    /// Returns a new function
    pub fn new(name: &str, ret: Type) -> Self {
        Self {
            name: String::from(name),
            doc: None,
            params: Vec::new(),
            ret,
            attributes: Vec::new(),
            is_static: false,
            is_inline: false,
            is_extern: false,
            body: Vec::new(),
        }
    }

    /// returns the declaration for this function without the body
    pub fn to_decl(&self) -> Self {
        Self {
            name: self.name.clone(),
            doc: None,
            params: self.params.clone(),
            ret: self.ret.clone(),
            attributes: Vec::new(),
            is_static: self.is_static,
            is_inline: self.is_inline,
            is_extern: self.is_extern,
            body: Vec::new(),
        }
    }

    /// obtains the type for this function
    pub fn to_type(&self) -> Type {
        panic!("needs to implement a corresponding type.")
    }

    /// obtains a type reference of the return type
    pub fn ret_type(&self) -> &Type {
        &self.ret
    }

    /// Adds a new documentation to the function
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a new doc string to the function
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// creates a new parameter for the function
    pub fn new_param(&mut self, name: &str, ty: Type) -> &mut FunctionParam {
        self.params.push(FunctionParam::new(name, ty));
        self.params.last_mut().unwrap()
    }

    /// Push a param to the function's parameters
    pub fn push_param(&mut self, item: FunctionParam) -> &mut Self {
        self.params.push(item);
        self
    }

    /// obtains the parameter of the function
    pub fn get_param(&self, idx: usize) -> Option<&FunctionParam> {
        if idx < self.params.len() {
            Some(&self.params[idx])
        } else {
            None
        }
    }

    /// obtains a param by name
    pub fn get_param_by_name(&self, name: &str) -> Option<&FunctionParam> {
        for p in &self.params {
            if p.name() == name {
                return Some(p);
            }
        }
        None
    }

    /// adds a new attribute to the function
    pub fn push_attribute(&mut self, attr: &str) -> &mut Self {
        self.attributes.push(String::from(attr));
        self
    }

    /// sets the function to be static
    ///
    /// # Example
    ///
    /// void foo()   -> static void foo()
    pub fn set_static(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_extern = false;
        }
        self.is_static = val;
        self
    }

    /// sets the function to be static
    pub fn sstatic(&mut self) -> &mut Self {
        self.set_static(true)
    }

    /// sets the function to be inline
    ///
    /// # Example
    ///
    /// void foo()   -> inline void foo()
    pub fn set_inline(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_extern = false;
        }
        self.is_inline = true;
        self
    }

    /// makes the function to be an inline method
    pub fn inline(&mut self) -> &mut Self {
        self.set_inline(true)
    }

    /// sets the function to be extern
    ///
    /// # Example
    ///
    /// void foo()   ->  extern void foo()
    pub fn set_extern(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_inline = false;
            self.is_extern = false;
        }
        self.is_extern = val;
        self
    }

    /// makes the function to be an inline method
    pub fn eextern(&mut self) -> &mut Self {
        self.set_extern(true)
    }

    /// sets the body for the function
    pub fn set_body(&mut self, body: Vec<Stmt>) -> &mut Self {
        if !body.is_empty() {
            self.is_extern = false;
        }
        self.body = body;
        self
    }

    /// pushes a new statement to the function
    pub fn push_stmt(&mut self, stmt: Stmt) -> &mut Self {
        self.is_extern = false;
        self.body.push(stmt);
        self
    }

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

        if self.is_inline {
            write!(fmt, "inline ")?;
        }

        // the type
        self.ret.fmt(fmt)?;

        write!(fmt, " {}(", self.name)?;
        if self.params.is_empty() {
            write!(fmt, "void")?;
        } else {
            for (i, f) in self.params.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?;
                }
                f.fmt(fmt)?;
            }
        }
        write!(fmt, ")")?;

        if !self.attributes.is_empty() {
            write!(fmt, "__attribute__() // TODO")?;
        }

        // if there is no body, and is inline or we only want the declaration
        if !self.body.is_empty() && (!decl_only || self.is_inline) {
            fmt.block(|fmt| {
                for stmt in &self.body {
                    stmt.fmt(fmt)?;
                }
                Ok(())
            })?;
            writeln!(fmt)
        } else {
            writeln!(fmt, ";")
        }
    }

    /// formats the function definitions
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, false)
    }

    /// formats only the function declaration
    pub fn fmt_decl(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, true)
    }

    /// formats only the function definition
    pub fn fmt_def(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // inline functions are defined in the declaratin
        if self.is_inline {
            return Ok(());
        }
        self.do_fmt(fmt, false)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
