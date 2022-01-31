// cgen-rs
//
//
// MIT License
//
// Copyright (c) 2021 Reto Achermann
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

//! # Unions
//!
//! This module defines the C union. For now, this is just supporting standard
//! C union.

// TODO: see if this can be merged with the struct type

use std::fmt::{self, Display, Write};

use crate::{BaseType, Doc, Field, Formatter, Type};

///defines a union
#[derive(Debug, Clone)]
pub union Union {
    /// the name of the union
    name: String,

    /// the fields of the union
    fields: Vec<Field>,

    /// the documentation for this union
    doc: Option<Doc>,

    /// attributes for the union
    attributes: Vec<String>,
}

impl Struct {
    /// Returns a new `Enum` instance with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: Vec::new(),
            doc: None,
            attributes: Vec::new(),
        }
    }

    /// Returns the corresponding type reference for this union
    ///
    /// # Example
    ///
    /// union Foo {}  => union Foo;
    pub fn as_type(&self) -> Type {
        Type::new(BaseType::Union(self.name.clone()))
    }

    /// Adds a new documentation to the enum
    pub fn doc(&mut self, doc: Doc) -> &mut Self {
        self.doc = Some(doc);
        self
    }

    /// Adds a new doc string to the enum
    pub fn push_doc_str(&mut self, doc: &str) -> &mut Self {
        if let Some(d) = &mut self.doc {
            d.add_text(doc);
        } else {
            self.doc = Some(Doc::with_str(doc));
        }
        self
    }

    /// creates a new variant with the given name and value
    pub fn new_field(&mut self, name: &str, ty: Type) -> &mut Field {
        self.fields.push(Field::new(name, ty));
        self.fields.last_mut().unwrap()
    }

    /// Push a variant to the enum.
    pub fn push_field(&mut self, item: Field) -> &mut Self {
        self.fields.push(item);
        self
    }

    /// adds a new attribute to the union
    pub fn push_attribute(&mut self, attr: &str) -> &mut Self {
        self.attributes.push(String::from(attr));
        self
    }

    /// Formats the enum using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(ref docs) = self.doc {
            docs.fmt(fmt)?;
        }

        write!(fmt, "union {}", self.name)?;

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
