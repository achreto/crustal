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

//! # Class Constructors and Destructors
//!
//! This module contains definitions for C++ class constructors and destructors

use std::fmt::{self, Write};

use crate::{BaseType, Doc, Expr, Formatter, MethodParam, Stmt, Type, Visibility};

/// holds a method definition
#[derive(Debug, Clone)]
pub struct Constructor {
    /// Name of the method
    name: String,

    /// the visibility of the method
    visibility: Visibility,

    /// the method documentation
    doc: Option<Doc>,

    /// the method arguments
    args: Vec<MethodParam>,

    /// the initalizer list
    initializers: Vec<Expr>,

    /// this is the default constructor
    is_default: bool,

    /// marks the constructor as deleted
    is_delete: bool,

    /// this is a copy constructor
    is_copy: bool,

    /// this is a move contstructor
    is_move: bool,

    /// wheter the definition is inside of the class
    is_inside: bool,

    /// the body of the method, a sequence of statements
    body: Vec<Stmt>,
}

impl Constructor {
    /// Creates a new constructor definition
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            doc: None,
            visibility: Visibility::Public,
            args: Vec::new(),
            initializers: Vec::new(),
            is_default: false,
            is_delete: false,
            is_copy: false,
            is_move: false,
            is_inside: false,
            body: Vec::new(),
        }
    }

    /// creates a new move constructor
    pub fn new_move(name: &str) -> Self {
        let mut c = Constructor::new(name);
        c.is_move = true;
        c
    }

    /// creates a new copy constructor
    pub fn new_copy(name: &str) -> Self {
        let mut c = Constructor::new(name);
        c.is_copy = true;
        c
    }

    /// creates a new copy constructor
    pub fn new_default(name: &str) -> Self {
        let mut c = Constructor::new(name);
        c.is_default = true;
        c
    }

    /// creates a new copy constructor
    pub fn new_delete(name: &str) -> Self {
        let mut c = Constructor::new(name);
        c.is_delete = true;
        c
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
    pub fn push_argument(&mut self, arg: MethodParam) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn new_argument(&mut self, name: &str, ty: Type) -> &mut MethodParam {
        self.push_argument(MethodParam::new(name, ty));
        self.args.last_mut().unwrap()
    }

    /// pushes a new elemenet to the initializer list
    pub fn push_initializer(&mut self, field_name: &str, value: Expr) -> &mut Self {
        self.initializers.push(Expr::FnCall {
            name: String::from(field_name),
            args: vec![value],
        });
        self
    }

    pub fn push_parent_initializer(&mut self, value: Expr) -> &mut Self {
        self.initializers.push(value);
        self
    }

    /// sets the constructor to be default
    ///
    /// # Example
    ///
    /// Foo()   -> Foo() = default
    pub fn set_default(&mut self, val: bool) -> &mut Self {
        if val {
            self.body.clear();
            if !self.is_copy {
                self.args.clear();
            }
            self.is_delete = false;
        }
        self.is_default = val;
        self
    }

    /// makes the constructor the default one
    pub fn default(&mut self) -> &mut Self {
        self.set_default(true)
    }

    /// sets the constructor to be deleted
    ///
    /// # Example
    ///
    /// Foo()   -> Foo() = delete;
    pub fn set_delete(&mut self, val: bool) -> &mut Self {
        if val {
            self.body.clear();
            if !self.is_copy {
                self.args.clear();
            }
            self.is_default = false;
        }
        self.is_delete = val;
        self
    }

    /// makes the constructor the default one
    pub fn delete(&mut self) -> &mut Self {
        self.set_delete(true)
    }

    /// marks this constructor as the copy constructor
    ///
    /// # Example
    ///
    /// Foo()   -> Foo(const Foo&)
    pub fn set_copy(&mut self, val: bool) -> &mut Self {
        if val {
            let mut ty = Type::new(BaseType::Class(self.name.clone()));
            ty.constant().reference();
            self.args = vec![MethodParam::new("other", ty)];
        }
        self.is_copy = val;
        self
    }

    /// makes the method to be an static method
    pub fn copy(&mut self) -> &mut Self {
        self.set_copy(true)
    }

    /// marks this constructor as the move constructor
    ///
    /// # Example
    ///
    /// Foo()   -> Foo(Foo &&)
    pub fn set_move(&mut self, val: bool) -> &mut Self {
        if val {
            let mut ty = Type::new(BaseType::Class(self.name.clone()));
            ty.reference().reference();
            self.args = vec![MethodParam::new("other", ty)];
        }
        self.is_move = val;
        self
    }

    /// makes the method to be an static method
    pub fn movec(&mut self) -> &mut Self {
        self.set_move(true)
    }

    /// sets the definition localtion of the method
    pub fn set_inside_def(&mut self, val: bool) -> &mut Self {
        self.is_inside = val;
        self
    }

    /// this method is defined inside
    pub fn inside_def(&mut self) -> &mut Self {
        self.set_inside_def(true)
    }

    /// sets the body for the method
    pub fn set_body(&mut self, body: Vec<Stmt>) -> &mut Self {
        if !body.is_empty() {
            self.is_default = false;
            self.is_delete = false;
        }
        self.body = body;
        self
    }

    /// pushes a new statement to the method
    pub fn push_stmt(&mut self, stmt: Stmt) -> &mut Self {
        self.is_default = false;
        self.is_delete = false;
        self.body.push(stmt);
        self
    }

    /// Formats the attribute using the given formatter.
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if !self.body.is_empty() | self.doc.is_some() {
            writeln!(fmt)?;
        }

        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if decl_only {
            write!(fmt, "{}", self.name)?;
        } else {
            fmt.write_scoped_name(self.name.as_str())?;
        }

        if self.args.is_empty() {
            write!(fmt, "(void)")?;
        } else {
            write!(fmt, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?;
                }
                arg.do_fmt(fmt, decl_only)?;
            }
            write!(fmt, ")")?;
        }

        if self.is_default {
            return writeln!(fmt, " = default;");
        }

        if self.is_delete {
            return writeln!(fmt, " = delete;");
        }

        // if we want to have the declaration only, then do that,
        // but only if it's not a inside method or an inline method
        if decl_only && !(self.is_inside) {
            return writeln!(fmt, ";");
        }

        writeln!(fmt)?;
        if !self.initializers.is_empty() && (!decl_only || self.is_inside) {
            fmt.indent(|fmt| {
                write!(fmt, ": ").expect("initializer");
                for (i, e) in self.initializers.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ").expect("initializer");
                    }
                    e.fmt(fmt).expect("initializer");
                    writeln!(fmt).expect("initializer");
                }
            })
        }

        writeln!(fmt, "{{")?;
        fmt.indent(|f| {
            for stmt in &self.body {
                stmt.fmt(f)?;
            }
            Ok(())
        })?;
        writeln!(fmt, "}}\n")
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
        if self.is_inside {
            return Ok(());
        }
        self.do_fmt(fmt, false)
    }
}

#[derive(Debug, Clone)]
pub struct Destructor {
    /// Name of the method
    name: String,

    /// the method documentation
    doc: Option<Doc>,

    /// this is the default constructor
    is_default: bool,

    /// marks the constructor as deleted
    is_delete: bool,

    /// wheter the definition is inside of the class
    is_inside: bool,

    /// sets the pure
    is_pure: bool,

    /// the body of the method, a sequence of statements
    body: Vec<Stmt>,
}

impl Destructor {
    /// Creates a new constructor definition
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            doc: None,
            is_default: false,
            is_delete: false,
            is_inside: false,
            is_pure: false,
            body: Vec::new(),
        }
    }

    /// creates a new move constructor
    pub fn new_delete(name: &str) -> Self {
        let mut c = Destructor::new(name);
        c.is_delete = true;
        c
    }

    /// creates a new copy constructor
    pub fn new_default(name: &str) -> Self {
        let mut c = Destructor::new(name);
        c.is_default = true;
        c
    }

    /// creates a new copy constructor
    pub fn new_pure(name: &str) -> Self {
        let mut c = Destructor::new(name);
        c.is_pure = true;
        c
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

    /// sets the constructor to be default
    ///
    /// # Example
    ///
    /// Foo()   -> Foo() = default
    pub fn set_default(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_delete = false;
        }
        self.is_default = val;
        self
    }

    /// makes the constructor the default one
    pub fn default(&mut self) -> &mut Self {
        self.set_default(true)
    }

    /// sets the constructor to be deleted
    ///
    /// # Example
    ///
    /// Foo()   -> Foo() = delete;
    pub fn set_delete(&mut self, val: bool) -> &mut Self {
        if val {
            self.is_default = false;
        }
        self.is_delete = val;
        self
    }

    /// makes the constructor the default one
    pub fn delete(&mut self) -> &mut Self {
        self.set_delete(true)
    }

    /// sets the definition localtion of the method
    pub fn set_inside_def(&mut self, val: bool) -> &mut Self {
        self.is_inside = val;
        self
    }

    /// this method is defined inside
    pub fn inside_def(&mut self) -> &mut Self {
        self.set_inside_def(true)
    }

    /// sets the method to be pure
    ///
    /// # Example
    ///
    /// void foo()   -> virtual void foo() = 0
    pub fn set_pure(&mut self, val: bool) -> &mut Self {
        if val {
            self.body.clear();
        }
        self.is_pure = val;
        self
    }

    /// turns the method into a pure method
    pub fn pure(&mut self) -> &mut Self {
        self.set_pure(true)
    }

    /// sets the body for the method
    pub fn set_body(&mut self, body: Vec<Stmt>) -> &mut Self {
        if !body.is_empty() {
            self.is_default = false;
            self.is_delete = false;
            self.is_pure = false;
        }
        self.body = body;
        self
    }

    /// pushes a new statement to the method
    pub fn push_stmt(&mut self, stmt: Stmt) -> &mut Self {
        self.is_default = false;
        self.is_delete = false;
        self.is_pure = false;
        self.body.push(stmt);
        self
    }

    /// Formats the attribute using the given formatter.
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if !self.body.is_empty() | self.doc.is_some() {
            writeln!(fmt)?;
        }

        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if self.is_pure {
            write!(fmt, "virtual ")?;
        }

        write!(fmt, "~{}(void)", self.name)?;

        if self.is_default {
            return writeln!(fmt, " = default;");
        }

        if self.is_delete {
            return writeln!(fmt, " = delete;");
        }

        if self.is_pure {
            return writeln!(fmt, " = 0;");
        }

        // if we want to have the declaration only, then do that,
        // but only if it's not a inside method or an inline method
        if self.body.is_empty() || (decl_only && !(self.is_inside)) {
            return writeln!(fmt, ";");
        }

        writeln!(fmt, " {{")?;
        fmt.indent(|f| {
            for stmt in &self.body {
                stmt.fmt(f)?;
            }
            Ok(())
        })?;
        writeln!(fmt, "}}\n")
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
        if self.is_inside {
            return Ok(());
        }
        self.do_fmt(fmt, false)
    }
}
