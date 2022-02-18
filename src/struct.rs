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

//! # Structs
//!
//! This module defines the C struct. For now, this is just supporting standard
//! C structs, for C++ structs use the 'class' module.
//!
//! Right now nested, anonymous structs cannot be supported. However, you can define
//! a `Field` that has a struct type.

use std::fmt::{self, Display, Write};

use crate::{Doc, Field, Formatter, Type};

///defines a struct
#[derive(Debug, Clone)]
pub struct Struct {
    /// the name of the struct
    name: String,

    /// the fields of the struct
    fields: Vec<Field>,

    /// the documentation for this struct
    doc: Option<Doc>,

    /// attributes for the struct
    attributes: Vec<String>,
}

impl Struct {
    /// Returns a new `Struct` instance with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: Vec::new(),
            doc: None,
            attributes: Vec::new(),
        }
    }

    /// Creates a new `Struct` with the given name and the supplied fields
    ///
    /// Note: the fields are not checked for duplicates.
    pub fn with_fields(name: &str, fields: Vec<Field>) -> Self {
        Self {
            name: String::from(name),
            fields,
            doc: None,
            attributes: Vec::new(),
        }
    }

    /// Returns the corresponding type reference for this struct
    ///
    /// # Example
    ///
    /// struct Foo {}  => struct Foo;
    pub fn to_type(&self) -> Type {
        Type::new_struct(&self.name)
    }

    /// Adds a new documentation to the struct
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a new doc string to the struct
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// creates a new field with the given name and value
    ///
    /// Note: the field is not checked for duplicates.
    pub fn new_field(&mut self, name: &str, ty: Type) -> &mut Field {
        self.fields.push(Field::new(name, ty));
        self.fields.last_mut().unwrap()
    }

    /// Push a field to the struct.
    ///
    /// Note: the field is not checked for duplicates.
    pub fn push_field(&mut self, item: Field) -> &mut Self {
        self.fields.push(item);
        self
    }

    /// obtains a reference to the field with the given name
    pub fn field_by_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name() == name)
    }

    /// obtains a mutable reference to the field with the given name
    pub fn field_by_name_mut(&mut self, name: &str) -> Option<&mut Field> {
        self.fields.iter_mut().find(|f| f.name() == name)
    }

    /// obtains a reference to the field with the given index (starting at 0)
    pub fn field_by_idx(&self, idx: usize) -> Option<&Field> {
        self.fields.get(idx)
    }

    /// obtains a mutable reference to the field with the given index mut
    pub fn field_by_idx_mut(&mut self, idx: usize) -> Option<&mut Field> {
        self.fields.get_mut(idx)
    }

    /// adds a new attribute to the struct
    pub fn push_attribute(&mut self, attr: String) -> &mut Self {
        self.attributes.push(attr);
        self
    }

    /// Formats a forward declaration for the struct
    pub fn fmt_decl(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "struct {};   // forward declaration", self.name)
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        write!(fmt, "struct {}", self.name)?;

        // consider this as a forward declaration
        if !self.fields.is_empty() {
            fmt.block(|fmt| {
                for field in &self.fields {
                    field.fmt(fmt)?;
                }
                Ok(())
            })?;

            if !self.attributes.is_empty() {
                write!(fmt, "__attribute__() // TODO")?;
            }
        }

        writeln!(fmt, ";")
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
