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

//! # Block
//!
//! This module defines a block of code in function bodies, loop bodies, or conditional branches

// std includes
use std::fmt::{self, Write};

use crate::{Comment, DoWhileLoop, Expr, ForLoop, Formatter, IfElse, Type, Variable, WhileLoop};

/// defines an item of the scope
#[derive(Debug, Clone)]
enum Item {
    Comment(Comment),
    Variable(Variable),
    IfElse(IfElse),
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    Return(Option<Expr>),
    Assign(Expr, Expr),
    GoTo(String),
    Label(String),
    Raw(String),
    FnCall(String, Vec<Expr>),
    MethodCall(Expr, String, Vec<Expr>),
    Break,
    Continue,
    NewLine,
}

/// defines the scope of the generated C code
#[derive(Debug, Clone)]
pub struct Block {
    /// items of this scope
    items: Vec<Item>,
}

impl Block {
    /// creates a new, empty block of statements
    pub fn new() -> Self {
        Block { items: Vec::new() }
    }

    /// merge two blocks
    pub fn merge(&mut self, other: Block) {
        let mut other = other;
        self.items.append(other.items.as_mut());
    }

    /// checks whether the body is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// clears the body
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// adds an additional empty line in the block
    pub fn empty_line(&mut self) -> &mut Self {
        self.items.push(Item::NewLine);
        self
    }

    /// adds a `break` statement to the block
    pub fn break_stmt(&mut self) -> &mut Self {
        self.items.push(Item::Break);
        self
    }

    /// adds a "raw" statement to the block, copying the string
    pub fn raw_str(&mut self, raw: &str) -> &mut Self {
        self.items.push(Item::Raw(String::from(raw)));
        self
    }

    /// pushs a "raw" statement to the block
    pub fn raw(&mut self, raw: String) -> &mut Self {
        self.items.push(Item::Raw(raw));
        self
    }

    /// pushes an assignment operation to the block
    pub fn assign(&mut self, lhs: Expr, rhs: Expr) -> &mut Self {
        self.items.push(Item::Assign(lhs, rhs));
        self
    }

    /// adds a new label to the block
    pub fn label(&mut self, label: &str) -> &mut Self {
        self.items.push(Item::Label(String::from(label)));
        self
    }

    /// adds a new goto to the block
    pub fn goto(&mut self, label: &str) -> &mut Self {
        self.items.push(Item::GoTo(String::from(label)));
        self
    }

    /// adds a `continue` statement to the block
    pub fn continue_stmt(&mut self) -> &mut Self {
        self.items.push(Item::Continue);
        self
    }

    /// adds a new comment from string to the scope
    pub fn new_comment(&mut self, comment: &str) -> &mut Self {
        self.comment(Comment::with_str(comment));
        self
    }

    /// adds a comment to the scope
    pub fn comment(&mut self, comment: Comment) -> &mut Self {
        self.items.push(Item::Comment(comment));
        self
    }

    /// adds a new if/else contitional to the block
    pub fn new_ifelse(&mut self, cond: &Expr) -> &mut IfElse {
        let ifelse = IfElse::new(cond);
        self.items.push(Item::IfElse(ifelse));
        match *self.items.last_mut().unwrap() {
            Item::IfElse(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// adds an ifelse conditional to the block
    pub fn ifelse(&mut self, s: IfElse) -> &mut Self {
        self.items.push(Item::IfElse(s));
        self
    }

    /// adds a new for loop to the block
    pub fn new_for_loop(&mut self, init: &Expr, guard: &Expr, step: &Expr) -> &mut ForLoop {
        let forloop = ForLoop::from_expr(init, guard, step);
        self.items.push(Item::ForLoop(forloop));
        match *self.items.last_mut().unwrap() {
            Item::ForLoop(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// adds a for loop to the block
    pub fn for_loop(&mut self, s: ForLoop) -> &mut Self {
        self.items.push(Item::ForLoop(s));
        self
    }

    /// adds a new while loop to the block
    pub fn new_while_loop(&mut self, cond: &Expr) -> &mut WhileLoop {
        self.while_loop(WhileLoop::new(cond));
        match *self.items.last_mut().unwrap() {
            Item::WhileLoop(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// adds a while loop to the block
    pub fn while_loop(&mut self, s: WhileLoop) -> &mut Self {
        self.items.push(Item::WhileLoop(s));
        self
    }

    /// adds a new `do-while` loop to the block
    pub fn new_dowhile_loop(&mut self, cond: &Expr) -> &mut DoWhileLoop {
        self.dowhile_loop(DoWhileLoop::new(cond));
        match *self.items.last_mut().unwrap() {
            Item::DoWhileLoop(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// adds a do-while loop to the block
    pub fn dowhile_loop(&mut self, s: DoWhileLoop) -> &mut Self {
        self.items.push(Item::DoWhileLoop(s));
        self
    }

    /// adds a new variable to the scope
    pub fn new_variable(&mut self, name: &str, ty: Type) -> &mut Variable {
        self.variable(Variable::new(name, ty));
        match *self.items.last_mut().unwrap() {
            Item::Variable(ref mut v) => v,
            _ => unreachable!(),
        }
    }

    /// adding a variable to the block
    pub fn variable(&mut self, var: Variable) -> &mut Self {
        self.items.push(Item::Variable(var));
        self
    }

    /// return statement from a expression
    pub fn new_return(&mut self, expr: Option<&Expr>) -> &mut Self {
        if let Some(e) = expr {
            self.items.push(Item::Return(Some(e.clone())));
        } else {
            self.items.push(Item::Return(None));
        }

        self
    }

    /// return statement returning an value
    pub fn return_expr(&mut self, expr: Expr) -> &mut Self {
        self.items.push(Item::Return(Some(expr)));
        self
    }

    /// return statement returning nothing
    pub fn return_none(&mut self) -> &mut Self {
        self.items.push(Item::Return(None));
        self
    }

    /// a printf statement
    pub fn printf(&mut self, format: &str, vars: Vec<Expr>) -> &mut Self {
        let mut vars = vars;
        let mut args = vec![Expr::new_str(format)];
        args.append(&mut vars);
        self.items.push(Item::FnCall(String::from("printf"), args));
        self
    }

    /// adds a statement that prints the string
    pub fn printstr(&mut self, format: &str) -> &mut Self {
        self.items
            .push(Item::FnCall(String::from("printf"), vec![Expr::new_str(format)]));
        self
    }

    /// a function call
    pub fn fn_call(&mut self, name: &str, args: Vec<Expr>) -> &mut Self {
        self.items.push(Item::FnCall(String::from(name), args));
        self
    }

    /// a method call
    pub fn method_call(&mut self, obj: Expr, method: &str, args: Vec<Expr>) -> &mut Self {
        self.items.push(Item::MethodCall(obj, String::from(method), args));
        self
    }

    /// formats the block
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        for item in self.items.iter() {
            match &item {
                Item::Comment(v) => v.fmt(fmt)?,
                Item::NewLine => writeln!(fmt)?,
                Item::Break => writeln!(fmt, "break;")?,
                Item::Continue => writeln!(fmt, "continue;")?,
                Item::Raw(v) => writeln!(fmt, "{};", v)?,
                Item::Label(v) => writeln!(fmt, "{}:", v)?,
                Item::GoTo(v) => writeln!(fmt, "goto {};", v)?,
                Item::Assign(l, r) => {
                    l.fmt(fmt)?;
                    write!(fmt, " = ")?;
                    r.fmt(fmt)?;
                    writeln!(fmt, ";")?;
                }
                Item::IfElse(v) => v.fmt(fmt)?,
                Item::ForLoop(v) => v.fmt(fmt)?,
                Item::WhileLoop(v) => v.fmt(fmt)?,
                Item::DoWhileLoop(v) => v.fmt(fmt)?,
                Item::Variable(v) => v.fmt(fmt)?,
                Item::Return(None) => writeln!(fmt, "return;")?,
                Item::Return(Some(v)) => {
                    write!(fmt, "return ")?;
                    v.fmt(fmt)?;
                    writeln!(fmt, ";")?
                }
                Item::FnCall(name, args) => {
                    write!(fmt, "{}(", name)?;
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(fmt, ", ")?;
                        }
                        arg.fmt(fmt)?;
                    }
                    writeln!(fmt, ");")?
                }
                Item::MethodCall(obj, method, args) => {
                    obj.fmt(fmt)?;
                    if obj.is_ptr() {
                        write!(fmt, "->{}(", method)?;
                    } else {
                        write!(fmt, ".{}(", method)?;
                    }

                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            write!(fmt, ", ")?;
                        }
                        arg.fmt(fmt)?;
                    }
                    writeln!(fmt, ");")?
                }
            }
        }

        Ok(())
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = String::new();
        self.fmt(&mut Formatter::new(&mut ret)).unwrap();
        write!(f, "{}", ret)
    }
}
