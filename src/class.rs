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

//! # Classes
//!
//! The `Class` module provides functionality to express a C++ class with its
//! attributes and method members.
//!
//! The class implementation does currently not support multiple inheritance,
//! or the definition of nested types, or generics.

use std::fmt::{self, Display, Write};

use crate::{
    Attribute, BaseType, Constructor, Destructor, Doc, Formatter, Method, Type, Visibility,
};

/// Defines a C++ class
#[derive(Debug, Clone)]
pub struct Class {
    /// Name of the class
    name: String,

    /// Documentation comment of the class
    doc: Option<Doc>,

    /// Parent class with its visibility
    base: Option<(Visibility, String)>,

    /// Class constructor methods
    constructors: Vec<Constructor>,

    /// Class destructor methods
    destructor: Option<Destructor>,

    /// Method members of the class with their visibility
    methods: Vec<Method>,

    /// Field members of the class with their visibility
    attributes: Vec<Attribute>,
}

impl Class {
    /// Creates a new, empty class with the given `name`.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            doc: None,
            base: None,
            destructor: None,
            constructors: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
        }
    }

    /// Returns the corresponding type for this class
    ///
    /// # Example
    ///
    /// struct Foo {}  => struct Foo;
    pub fn to_type(&self) -> Type {
        Type::new(BaseType::Class(self.name.clone()))
    }

    /// adds a string to the documentation comment to the class
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// sets the documentation comment for the class
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// sets the base class
    pub fn set_base(&mut self, base: &str, visibility: Visibility) -> &mut Self {
        self.base = Some((visibility, base.to_string()));
        self
    }

    /// adds a new field member to the class with the given visibility
    pub fn new_attribute(&mut self, name: &str, ty: Type) -> &mut Attribute {
        self.attributes.push(Attribute::new(name, ty));
        self.attributes.last_mut().unwrap()
    }

    /// adds the field member to the class with the given visibility
    pub fn push_attribute(&mut self, field: Attribute) -> &mut Self {
        self.attributes.push(field);
        self
    }

    /// adds a new method member to the class with the given visibility
    pub fn new_method(&mut self, name: &str, ty: Type) -> &mut Method {
        self.methods.push(Method::new(name, ty));
        self.methods.last_mut().unwrap()
    }

    /// adds the field member to the class with the given visibility
    pub fn push_method(&mut self, method: Method) -> &mut Self {
        self.methods.push(method);
        self
    }

    pub fn new_constructor(&mut self) -> &mut Constructor {
        self.constructors.push(Constructor::new(self.name.as_str()));
        self.constructors.last_mut().unwrap()
    }

    /// creates a new destructor for the
    pub fn new_destructor(&mut self) -> &mut Destructor {
        self.destructor = Some(Destructor::new(self.name.as_str()));
        self.destructor.as_mut().unwrap()
    }

    pub fn do_fmt_class_scope(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        if !decl_only {
            self.constructors.iter().for_each(|m| {
                m.do_fmt(fmt, decl_only).expect("format failed");
            });

            self.attributes.iter().filter(|a| a.is_static()).for_each(|m| {
                m.do_fmt(fmt, decl_only).expect("format failed");
            });

            self.methods.iter().for_each(|m| {
                m.do_fmt(fmt, decl_only).expect("format failed");
            });

            return Ok(());
        }

        write!(fmt, "class {}", self.name)?;

        // the derived class
        if let Some(p) = &self.base {
            write!(fmt, " : {} {}", p.0, p.1)?;
        }

        let pub_attr = self.attributes.iter().filter(|a| a.is_public()).count();
        let pub_methods = self.methods.iter().filter(|a| a.is_public()).count();
        let pub_constructors = self.constructors.iter().filter(|a| a.is_public()).count();
        let prot_attr = self.attributes.iter().filter(|a| a.is_protected()).count();
        let prot_methods = self.methods.iter().filter(|a| a.is_protected()).count();
        let prot_constructors = self.constructors.iter().filter(|a| a.is_protected()).count();
        let priv_attr = self.attributes.iter().filter(|a| a.is_private()).count();
        let priv_methods = self.methods.iter().filter(|a| a.is_private()).count();
        let priv_constructors = self.constructors.iter().filter(|a| a.is_private()).count();

        if self.destructor.is_none()
            && pub_attr
                + pub_methods
                + pub_constructors
                + prot_attr
                + prot_methods
                + prot_constructors
                + priv_attr
                + priv_methods
                + priv_constructors
                == 0
        {
            return writeln!(fmt, " {{ }};");
        }

        fmt.block(|fmt| {
            if self.destructor.is_some() || pub_attr + pub_methods + pub_constructors > 0 {
                writeln!(fmt, "\npublic:")?;
            }

            if pub_constructors > 0 {
                self.constructors.iter().filter(|m| m.is_public()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if let Some(d) = &self.destructor {
                d.do_fmt(fmt, decl_only)?;
            }

            if pub_attr > 0 {
                self.attributes.iter().filter(|a| a.is_public()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if pub_methods > 0 {
                self.methods.iter().filter(|m| m.is_public()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if prot_attr + prot_attr + prot_constructors > 0 {
                writeln!(fmt, "\nprotected:")?;
            }

            if prot_constructors > 0 {
                self.constructors.iter().filter(|m| m.is_protected()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if prot_attr > 0 {
                self.attributes.iter().filter(|a| a.is_protected()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }
            if prot_methods > 0 {
                self.methods.iter().filter(|m| m.is_protected()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if priv_attr + priv_attr + priv_constructors > 0 {
                writeln!(fmt, "\nprivate:")?;
            }

            if priv_constructors > 0 {
                self.constructors.iter().filter(|m| m.is_private()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }

            if priv_attr > 0 {
                self.attributes.iter().filter(|a| a.is_private()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }
            if priv_methods > 0 {
                self.methods.iter().filter(|m| m.is_private()).for_each(|m| {
                    m.do_fmt(fmt, decl_only).expect("format failed");
                });
            }
            Ok(())
        })?;
        writeln!(fmt, ";")
    }

    /// formats the class
    pub fn do_fmt(&self, fmt: &mut Formatter<'_>, decl_only: bool) -> fmt::Result {
        fmt.scope(self.name.as_str(), |fmt| {
            self.do_fmt_class_scope(fmt, decl_only).expect("failed to format the class")
        });
        Ok(())
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
        self.do_fmt(fmt, false)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt_def(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)?;

        let mut ret = String::new();
        self.fmt_decl(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
