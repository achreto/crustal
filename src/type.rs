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

use std::fmt::{self, Display, Write};

use crate::formatter::Formatter;

/// Represents the visibility for C++ class members
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    /// Members are declared to be public
    Public,
    /// Members are declared to be protected
    Protected,
    /// Members are declared to be private
    Private,
    /// the default visibility
    Default,
}

impl Visibility {
    /// formats the visibility identifier
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use Visibility::*;
        match self {
            Public => write!(fmt, "public"),
            Protected => write!(fmt, "protected"),
            Private => write!(fmt, "private"),
            Default => Ok(()),
        }
    }
}

impl Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Visibility::*;
        match self {
            Public => write!(f, "public"),
            Protected => write!(f, "protected"),
            Private => write!(f, "private"),
            Default => Ok(()),
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
    UInt8,
    /// an unsigned two byte integer. (`uint16_t`)
    UInt16,
    /// an unsigned four byte integer. (`uint32_t`)
    UInt32,
    /// an unsigned eight byte integer. (`uint64_t`)
    UInt64,
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
    UIntPtr,
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeModifier {
    Ptr,
    Volatile,
    Const,
    Ref,
}

/// represents a complete type
#[derive(Debug, Clone)]
pub struct Type {
    /// the base type
    base: BaseType,
    /// the type modifiers of the base type
    mods: Vec<TypeModifier>,
    /// the number of pointers
    nptr: u8,
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
            UInt8 => write!(fmt, "uint8_t"),
            UInt16 => write!(fmt, "uint16_t"),
            UInt32 => write!(fmt, "uint32_t"),
            UInt64 => write!(fmt, "uint64_t"),
            Int8 => write!(fmt, "int8_t"),
            Int16 => write!(fmt, "int16_t"),
            Int32 => write!(fmt, "int32_t"),
            Int64 => write!(fmt, "int64_t"),
            Size => write!(fmt, "size_t"),
            UIntPtr => write!(fmt, "uintptr_t"),
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

    /// formats the basetype into the supplied formatter
    pub fn is_integer(&self) -> bool {
        use BaseType::*;
        matches!(
            self,
            Char |
            UInt8  |
            UInt16 |
            UInt32 |
            UInt64 |
            Int8 |
            Int16 |
            Int32 |
            Int64 |
            Size |
            UIntPtr |
            Bool |
            // allowing the typedef here
            TypeDef(_)
        )
    }

    pub fn is_struct(&self) -> bool {
        use BaseType::*;
        matches!(self, Struct(_) | Union(_) | Class(_, _) | TypeDef(_))
    }

    /// creates a new integer type with a given type
    pub fn new_int(bits: u64) -> BaseType {
        use BaseType::*;
        match bits {
            8 => UInt8,
            16 => UInt16,
            32 => UInt32,
            64 => UInt64,
            _ => unreachable!(),
        }
    }
}

impl TypeModifier {
    /// formats the type modifier into the supplied formatter
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use TypeModifier::*;
        match self {
            Ptr => write!(fmt, " *"),
            Volatile => write!(fmt, " volatile"),
            Const => write!(fmt, " const"),
            Ref => write!(fmt, " &"),
        }
    }
}

impl Type {
    /// creates a new type from the base type
    pub fn new(base: BaseType) -> Self {
        Type {
            base,
            mods: Vec::new(),
            nptr: 0,
            is_volatile: false,
            is_const: false,
        }
    }

    pub fn new_int(bits: u64) -> Self {
        Type::new(BaseType::new_int(bits))
    }

    pub fn new_bool() -> Self {
        Type::new(BaseType::Bool)
    }

    pub fn new_size() -> Self {
        Type::new(BaseType::Size)
    }

    pub fn new_void() -> Self {
        Type::new(BaseType::Void)
    }

    pub fn new_typedef(name: &str) -> Self {
        Type::new(BaseType::TypeDef(name.to_string()))
    }

    /// creates a new type for the class
    pub fn new_class(classname: &str) -> Self {
        Type::new(BaseType::Class(classname.to_string(), Vec::new()))
    }

    /// obtainst the base type of the type
    pub fn basetype(&self) -> &BaseType {
        &self.base
    }

    /// checks if the type is a struct type
    pub fn is_struct(&self) -> bool {
        self.base.is_struct()
    }

    /// returns true if the base type is an integer
    pub fn is_integer(&self) -> bool {
        if self.nptr != 0 {
            return false;
        }
        self.base.is_integer()
    }

    /// returns true if the type represents a pointer value
    pub fn is_ptr(&self) -> bool {
        if self.nptr > 0 {
            return true;
        }
        // typedefs may always be pointers
        if let BaseType::TypeDef(_) = &self.base {
            return true;
        }
        false
    }

    /// create a new type from by taking a pointer of it
    ///
    /// # Example
    ///
    /// `int` => `int *`
    pub fn from_ptr(&self) -> Self {
        assert!(self.nptr < 32);
        let mut n = self.clone();
        n.mods.push(TypeModifier::Ptr);
        n.nptr += 1;
        n
    }

    /// create a new type from by taking a reference of it
    ///
    /// # Example
    ///
    /// `int` => `int &`
    pub fn from_ref(&self) -> Self {
        let mut n = self.clone();
        n.mods.push(TypeModifier::Ref);
        n
    }

    /// obtais a new type by dereferencing the pointer type
    ///
    /// # Example
    ///
    /// `int **` => `int *`
    pub fn from_deref(&self) -> Option<Self> {
        if self.nptr == 0 {
            return None;
        }

        let mut n = Self::new(self.base.clone());
        n.is_const = self.is_const;
        n.is_volatile = self.is_volatile;
        for m in &self.mods {
            // add the modifiers and count the pointers
            // if we hit the number of pointers, and hit
            // another pointer, return.
            if *m == TypeModifier::Ptr {
                if n.nptr == self.nptr - 1 {
                    return Some(n);
                }
                n.nptr += 1;
            }
            n.mods.push(*m);
        }
        Some(n)
    }

    /// create a new type by it const
    ///
    /// # Example
    ///
    /// `int *` => `int * const`
    pub fn from_const(&mut self) -> Self {
        let mut n = self.clone();
        n.mods.push(TypeModifier::Const);
        n
    }

    /// create a new type by making it volatile
    ///
    /// # Example
    ///
    /// `int *` => `int * volatile`
    pub fn from_volatile(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Volatile);
        self
    }

    /// sets the type of the object to be volatile
    ///
    /// # Example
    ///
    /// `int *` => `volatile int *`
    pub fn volatile_value(&mut self, val: bool) -> &mut Self {
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
    pub fn const_value(&mut self, val: bool) -> &mut Self {
        self.is_const = val;
        if val {
            self.is_volatile = false;
        }
        self
    }

    /// makes the current type a pointer type
    ///
    /// # Example
    ///
    /// `int` => `int *`
    pub fn pointer(&mut self) -> &mut Self {
        assert!(self.nptr < 32);
        self.mods.push(TypeModifier::Ptr);
        self.nptr += 1;
        self
    }

    /// makes the current type a pointer type
    ///
    /// # Example
    ///
    /// `int` => `int &`
    pub fn reference(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Ref);
        self
    }

    /// makes the current type a const type
    ///
    /// # Example
    ///
    /// `int *` => `int * const`
    pub fn constant(&mut self) -> &mut Self {
        self.mods.push(TypeModifier::Const);
        self
    }

    /// makes the current  type volatile
    ///
    /// # Example
    ///
    /// `int *` => `int * volatile`
    pub fn volatile(&mut self) -> &mut Self {
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

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
