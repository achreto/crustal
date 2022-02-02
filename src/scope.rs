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

//! # Scope
//!
//! This module defines the scope that contains definitions, functions, ...

// std includes
use std::fmt::{self, Write};
use std::fs;
use std::path::Path;

use crate::{
    Class, Comment, Doc, Enum, Formatter, Function, IfDef, Macro, Struct, Type, Union, Variable,
};

/// defines an item of the scope
#[derive(Debug, Clone)]
pub enum Item {
    Comment(Comment),
    Enum(Enum),
    IfDef(IfDef),
    Macro(Macro),
    Struct(Struct),
    Union(Union),
    Function(Function),
    Class(Class),
    Variable(Variable),
}

/// defines the scope of the generated C code
#[derive(Debug, Clone)]
pub struct Scope {
    /// the header document comment
    doc: Option<Doc>,

    /// items of this scope
    items: Vec<Item>,

    /// the output file
    file: Option<String>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            doc: None,
            items: Vec::new(),
            file: None,
        }
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

    /// sets the output file path
    pub fn file(&mut self, file: &str) -> &mut Self {
        self.file = Some(String::from(file));
        self
    }

    /// adds a new comment to the scope
    pub fn new_comment(&mut self, comment: &str) -> &mut Comment {
        self.push_comment(Comment::new(comment));

        match *self.items.last_mut().unwrap() {
            Item::Comment(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a comment to the scope
    pub fn push_comment(&mut self, comment: Comment) -> &mut Self {
        self.items.push(Item::Comment(comment));
        self
    }

    /// adds a new enum to the scope
    pub fn new_enum(&mut self, name: &str) -> &mut Enum {
        self.push_enum(Enum::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Enum(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a enum to the scope
    pub fn push_enum(&mut self, def: Enum) -> &mut Self {
        self.items.push(Item::Enum(def));
        self
    }

    /// adds a new struct to the scope
    pub fn new_struct(&mut self, name: &str) -> &mut Struct {
        self.push_struct(Struct::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Struct(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a struct to the scope
    pub fn push_struct(&mut self, inc: Struct) -> &mut Self {
        self.items.push(Item::Struct(inc));
        self
    }

    /// adds a new class to the scope
    pub fn new_class(&mut self, name: &str) -> &mut Class {
        self.push_class(Class::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Class(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a class to the scope
    pub fn push_class(&mut self, c: Class) -> &mut Self {
        self.items.push(Item::Class(c));
        self
    }

    /// adds a new union to the scope
    pub fn new_union(&mut self, name: &str) -> &mut Union {
        self.push_union(Union::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Union(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a class to the scope
    pub fn push_union(&mut self, c: Union) -> &mut Self {
        self.items.push(Item::Union(c));
        self
    }

    /// adds a new union to the scope
    pub fn new_function(&mut self, name: &str, ty: Type) -> &mut Function {
        self.push_function(Function::new(name, ty));

        match *self.items.last_mut().unwrap() {
            Item::Function(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a class to the scope
    pub fn push_function(&mut self, c: Function) -> &mut Self {
        self.items.push(Item::Function(c));
        self
    }

    /// adds a new macro to the scope
    pub fn new_macro(&mut self, name: &str) -> &mut Macro {
        self.push_macro(Macro::new(name));

        match *self.items.last_mut().unwrap() {
            Item::Macro(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a macro to the scope
    pub fn push_macro(&mut self, mac: Macro) -> &mut Self {
        self.items.push(Item::Macro(mac));
        self
    }

    /// adds a new variable to the scope
    pub fn new_variable(&mut self, name: &str, ty: Type) -> &mut Variable {
        self.push_variable(Variable::new(name, ty));

        match *self.items.last_mut().unwrap() {
            Item::Variable(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a variable to the scope
    pub fn push_variable(&mut self, mac: Variable) -> &mut Self {
        self.items.push(Item::Variable(mac));
        self
    }

    /// adds a new variable to the scope
    pub fn new_ifdef(&mut self, sym: &str) -> &mut IfDef {
        self.push_ifdef(IfDef::new(sym));

        match *self.items.last_mut().unwrap() {
            Item::IfDef(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// pushes a variable to the scope
    pub fn push_ifdef(&mut self, ifdef: IfDef) -> &mut Self {
        self.items.push(Item::IfDef(ifdef));
        self
    }

    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, only_decls: bool) -> fmt::Result {
        // documentation and license information
        self.doc.as_ref().map(|d| d.fmt(fmt));
        writeln!(fmt, "\n")?;
        for (i, item) in self.items.iter().enumerate() {
            if i != 0 {
                writeln!(fmt, "\n")?;
            }

            match &item {
                Item::Comment(v) => v.fmt(fmt)?,
                Item::Struct(v) => v.fmt(fmt)?,
                Item::Macro(v) => v.fmt(fmt)?,
                Item::Enum(v) => v.fmt(fmt)?,
                Item::Variable(v) => v.fmt(fmt)?,
                Item::IfDef(v) => v.fmt(fmt)?,
                Item::Union(v) => v.fmt(fmt)?,
                Item::Function(v) => v.fmt(fmt)?,
                Item::Class(v) => v.fmt(fmt)?,
            }
        }

        Ok(())
    }

    /// Formats the scope using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.do_fmt(fmt, false)
    }

    pub fn to_file(&self, path: &str, only_decls: bool) -> std::io::Result<()> {
        // set the path to the file
        let file = Path::new(path);

        let mut ret = String::new();
        self.do_fmt(&mut Formatter::new(&mut ret), only_decls)
            .unwrap();

        // write the file, return IOError otherwise
        fs::write(file, ret.as_bytes())
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();

        // Remove the trailing newline
        if ret.as_bytes().last() == Some(&b'\n') {
            ret.pop();
        }

        write!(f, "{}", ret)
    }
}
