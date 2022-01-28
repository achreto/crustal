// C/C++ Code Generator For Rust
//
//
// MIT License
//
// Copyright (c) 2021, 2022 Reto Achermann (The University of British Columbia)
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

//! # Types Definitions
//!
//! This module provides functionality to express types in C/C++ programs.

use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Represents the visibility for C++ class members
#[derive(Debug, Clone)]
pub enum Visibility {
    /// Members are declared to be public
    Public,
    /// Members are declared to be protected
    Protected,
    /// Members are declared to be private
    Private,
}

impl Visibility {
    /// formats the visibility identifier
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Visibility::*;
        match self {
            Public => writeln!(fmt, "public"),
            Protected => writeln!(fmt, "protected"),
            Private => writeln!(fmt, "private"),
        }
    }
}

/// Represents a base type in C/C++.
#[derive(Debug, Clone)]
pub enum BaseType {
    /// void type. Used in function return values, or generic pointers (`void *`).
    Void,
    /// double precision floating point number.
    Double,
    /// single precision floating point number.
    Float,
    /// a character
    Char,
    /// an unsigned one byte integer. (`uint8_t`)
    Unit8,
    /// an unsigned two byte integer. (`uint16_t`)
    Uint16,
    /// an unsigned four byte integer. (`uint32_t`)
    Uint32,
    /// an unsigned eight byte integer. (`uint64_t`)
    Uint64,
    /// a signed one byte integer. (`int8_t`)
    Int8,
    /// a signed two byte integer. (`int16_t`)
    Int16,
    /// a signed four byte integer. (`int32_t`)
    Int32,
    /// a signed eight byte integer. (`int64_t`)
    Int64,
    /// a size type (`size_t`)
    Size,
    /// a pointer value (`uintptr_t`)
    UintPtr,
    /// a boolean value (`bool`)
    Bool,
    /// an enumeration type `enum STRING`
    Enum(String),
    /// a struct type `struct STRING`
    Struct(String),
    /// a union type `union STRING`
    Union(String),
    /// class with templates `Foo<T>`
    Class(String, Vec<String>),
    /// a typedef `foo_t`
    TypeDef(String),
}

/// the type modifiers
#[derive(Debug, Clone, Copy)]
pub enum TypeModifier {
    Ptr,
    Volatile,
    Const,
}

/// represents a complete type
#[derive(Debug, Clone)]
pub struct Type {
    /// the base type
    base: BaseType,
    /// the type modifiers of the base type
    mods: Vec<TypeModifier>,
    /// whether the type is const
    is_const: bool,
    /// whether the type is volatile
    is_volatile: bool,
}

impl BaseType {
    /// formats the basetype into the supplied formatter
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use BaseType::*;
        match self {
            Void => write!(fmt, "void"),
            Double => write!(fmt, "double"),
            Float => write!(fmt, "float"),
            Char => write!(fmt, "char"),
            Unit8 => write!(fmt, "uint8_t"),
            Uint16 => write!(fmt, "uint16_t"),
            Uint32 => write!(fmt, "uint32_t"),
            Uint64 => write!(fmt, "uint64_t"),
            Int8 => write!(fmt, "int8_t"),
            Int16 => write!(fmt, "int16_t"),
            Int32 => write!(fmt, "int32_t"),
            Int64 => write!(fmt, "int64_t"),
            Size => write!(fmt, "size_t"),
            UintPtr => write!(fmt, "uintptr_t"),
            Bool => write!(fmt, "bool"),
            Enum(s) => write!(fmt, "enum {}", s),
            Struct(s) => write!(fmt, "struct {}", s),
            Union(s) => write!(fmt, "union {}", s),
            Class(s, t) => {
                if !t.is_empty() {
                    write!(fmt, "{}<{}>", s, t.join(","))
                } else {
                    write!(fmt, "{}", s)
                }
            }
            TypeDef(s) => write!(fmt, "{}", s),
        }
    }
}

impl TypeModifier {
    /// formats the type modifier into the supplied formatter
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use TypeModifier::*;
        match self {
            Ptr => write!(fmt, "* "),
            Volatile => write!(fmt, "volatile "),
            Const => write!(fmt, "const "),
        }
    }
}

impl Type {
    /// creates a new type from the base type
    pub fn new(base: BaseType) -> Self {
        Type {
            base,
            mods: Vec::new(),
            is_volatile: false,
            is_const: false,
        }
    }

    /// sets the type of the object to be volatile
    ///
    /// # Example
    ///
    /// `int *` => `volatile int *`
    pub fn set_volatile(&mut self, val: bool) -> &mut Self {
        self.is_volatile = val;
        if val {
            self.is_const = false;
        }
        self
    }

    /// sets the type of the object to be const
    ///
    /// # Example
    ///
    /// `int *` => `const int *`
    pub fn set_const(&mut self, val: bool) -> &mut Self {
        self.is_const = val;
        if val {
            self.is_volatile = false;
        }
        self
    }

    /// adds a pointer type of the current type
    ///
    /// # Example
    ///
    /// `int` => `int *`
    pub fn ptr_of(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Ptr);
        self
    }

    /// adds a const modifier to the type
    ///
    /// # Example
    ///
    /// `int *` => `int * const`
    pub fn const_of(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Const);
        self
    }

    /// adds a volatile modifier to the type
    ///
    /// # Example
    ///
    /// `int *` => `int * volatile`
    pub fn volatile_of(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Volatile);
        self
    }

    /// Formats the variant using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if self.is_volatile {
            write!(fmt, "volatile ")?;
        }

        if self.is_const {
            write!(fmt, "const ")?;
        }

        self.base.fmt(fmt)?;

        for m in &self.mods {
            m.fmt(fmt)?
        }

        Ok(())
    }
}
