#![allow(dead_code)]

use std::collections::HashMap;

use crate::ast::{Declaration, Expr, AST};

pub struct Program(Vec<JS>);

impl From<AST> for Program {
    fn from(ast: AST) -> Self {
        let require_std = require("./std").constt("std");
        let module: Vec<JS> =
            ast.get_declarations().into_iter().map(JS::from).collect();
        Self(
            vec![require_std]
                .into_iter()
                .chain(module.into_iter())
                .collect(),
        )
    }
}

impl JS {
    fn constt(self, name: &str) -> Self {
        Self::Assign(format!("const {}", name), self.into())
    }

    fn this(self, name: &str) -> Self {
        Self::Assign(format!("this.{}", name), self.into())
    }

    fn exports(self) -> Self {
        Self::Assign("module.exports".into(), self.into())
    }

    fn new(name: &str, args: Vec<Self>) -> Self {
        Self::Kw("new".into(), Self::name(name).call(vec![]).into())
    }

    fn returns(self) -> Self {
        Self::Kw("return".into(), self.into())
    }

    fn call(self, args: Vec<Self>) -> Self {
        Self::Call(self.into(), args)
    }

    fn name(name: &str) -> Self {
        Self::Name(name.into())
    }

    fn function(self, name: &str, params: Vec<&str>) -> Self {
        Self::Function(
            name.into(),
            params.into_iter().map(String::from).collect(),
            self.into(),
        )
    }

    fn str(s: &str) -> JS {
        Self::Str(s.into())
    }
}

fn require(path: &str) -> JS {
    JS::Call(
        JS::Name("require".into()).into(),
        vec![JS::Str(path.into())],
    )
}

impl Into<String> for Program {
    fn into(self) -> String {
        let defs: Vec<String> =
            self.0.into_iter().map(|def| def.into()).collect();
        defs.join("\n\n")
    }
}

#[derive(PartialEq, Eq, Debug)]
enum JS {
    Assign(String, Box<Self>),  // something = body;
    Call(Box<Self>, Vec<Self>), // f (x) (y) (z)
    Kw(String, Box<Self>),      // new ... or typeof ...
    Function(String, Vec<String>, Box<Self>), // function name(x, y, z) { body }
    Func(Vec<String>, Box<Self>), // x => y => z => body
    Proc(Vec<Self>),            // { a; list; of; statements; }
    Object(HashMap<String, Self>), // { x: 1, y: "hello" }
    Name(String),               // x
    Str(String),                // "hello"
    Int(i32),                   // -42
}

impl From<Declaration> for JS {
    fn from(decl: Declaration) -> Self {
        match decl.expr.clone() {
            Expr::Func(ps, expr) => {
                let param = &ps[0].clone();
                Self::Proc(vec![Self::from(reduce_func(*ps, *expr)).returns()])
                    .function(&decl.id, vec![param])
            }
            Expr::Call(f, args) => {
                Self::Proc(vec![Self::from(Expr::Call(f, args)).returns()])
                    .function(&decl.id, vec![])
            }
            other => Self::from(other).constt(&decl.id),
        }
    }
}

fn reduce_func(ps: Vec<String>, expr: Expr) -> Expr {
    if ps.len() == 1 {
        expr
    } else {
        Expr::Func(ps.split_at(1).1.to_vec().into(), expr.into())
    }
}

impl From<Vec<Declaration>> for JS {
    fn from(decls: Vec<Declaration>) -> Self {
        Self::Proc(decls.into_iter().map(Self::from).collect())
    }
}

impl From<Expr> for JS {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Int(i) => Self::Int(i),
            Expr::Name(name) | Expr::ID(name) => Self::Name(name),
            Expr::Call(f, args) => Self::Call(
                Box::new(Self::from(*f)),
                args.into_iter().map(Self::from).collect(),
            ),
            Expr::Func(params, expr) => {
                Self::Func(*params, Box::new(Self::from(*expr)))
            }
        }
    }
}

impl Into<String> for JS {
    fn into(self) -> String {
        match self {
            Self::Call(f, args) => {
                let args: Vec<String> = args
                    .into_iter()
                    .map(|arg| {
                        let s: String = arg.into();
                        format!("({})", s)
                    })
                    .collect();
                let fargs = if args.is_empty() {
                    "()".into()
                } else {
                    args.join(" ")
                };
                format!("{} {}", f.bracketed_func(), fargs)
            }
            Self::Func(params, body) => {
                let body: String = (*body).into();
                format!("{} => {}", params.join(" => "), body)
            }
            Self::Name(name) => name,
            Self::Int(i) => i.to_string(),
            Self::Assign(left, expr) => {
                let s: String = (*expr).into();
                format!("{} = {};", left, s)
            }
            Self::Kw(kw, expr) => {
                let s: String = (*expr).into();
                format!("{} {}", kw, s)
            }
            Self::Function(name, args, expr) => match *expr {
                Self::Proc(ss) => format!(
                    "function {}({}) {}",
                    name,
                    args.join(", "),
                    <Self as Into<String>>::into(Self::Proc(ss))
                ),
                other => panic!("Unexpected function body {:?}", other),
            },
            Self::Proc(statements) => {
                let ss: Vec<String> =
                    statements.into_iter().map(|s| s.into()).collect();
                format!("{{ {} }}", ss.join(" "))
            }
            Self::Object(map) => {
                let pairs: Vec<String> = map
                    .into_iter()
                    .map(|(key, expr)| {
                        format!(
                            "{}: {}",
                            key,
                            <Self as Into<String>>::into(expr)
                        )
                    })
                    .collect();
                format!("{{{}}}", pairs.join(","))
            }
            Self::Str(s) => format!(r#""{}""#, s),
        }
    }
}

impl JS {
    fn bracketed_func(self) -> String {
        match self {
            Self::Func(params, body) => {
                let s: String = Self::Func(params, body).into();
                format!("({})", s)
            }
            other => other.into(),
        }
    }
}
