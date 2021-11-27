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

//! # Types
//!
//! The type module represents types

use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// represents the possible base types
#[derive(Debug, Clone)]
pub enum BaseType {
    Void,
    Double,
    Float,
    Char,
    Unit8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Size,
    UintPtr,
    Enum(String),
    Struct(String),
    TypeDef(String),
}

/// the type modifiers
#[derive(Debug, Clone, Copy)]
pub enum TypeModifier {
    Ptr,
    Const,
}

/// represents a complete type
#[derive(Debug, Clone)]
pub struct Type {
    /// the base type
    base: BaseType,
    /// the type modifiers of the base type
    mods: Vec<TypeModifier>,
    /// the type is volatile
    is_volatile: bool,
    /// the type is const
    is_const: bool,
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
            Enum(s) => write!(fmt, "enum {}", s),
            Struct(s) => write!(fmt, "struct {}", s),
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
    /// `int *` => `volatile *int`
    pub fn set_volatile(&mut self) -> &mut Self {
        self.is_volatile = true;
        self
    }

    /// sets the type of the object to be const
    ///
    /// # Example
    ///
    /// `int *` => `const int *`
    pub fn set_const(&mut self) -> &mut Self {
        self.is_const = true;
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
