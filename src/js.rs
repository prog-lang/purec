use crate::ast::{Declaration, Expr, AST};

pub struct Program(Vec<JS>);

impl From<AST> for Program {
    fn from(ast: AST) -> Self {
        Self(ast.get_declarations().into_iter().map(JS::from).collect())
    }
}

impl Into<String> for Program {
    fn into(self) -> String {
        let mut module = vec![r#"const std = require("./std");"#.to_string()];
        let mut defs = self.0.into_iter().map(|def| def.into()).collect();
        module.append(&mut defs);
        module.push("main();".into());
        module.join("\n\n")
    }
}

enum JS {
    Def { name: String, expr: Box<Self> },
    Call(Box<Self>, Vec<Self>),
    Func(Vec<String>, Box<Self>),
    Name(String),
    Int(i32),
}

impl From<Declaration> for JS {
    fn from(decl: Declaration) -> Self {
        Self::Def {
            name: decl.id,
            expr: Box::new(decl.expr.into()),
        }
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
            JS::Def { name, expr } => {
                let body: String = (*expr).into();
                format!("const {} = {};", name, body)
            }
            JS::Call(f, args) => {
                let args: Vec<String> = args
                    .into_iter()
                    .map(|arg| {
                        let s: String = arg.into();
                        format!("({})", s)
                    })
                    .collect();
                format!("{} {}", f.bracketed_func(), args.join(" "))
            }
            JS::Func(params, body) => {
                let body: String = (*body).into();
                format!("{} => {}", params.join(" => "), body)
            }
            JS::Name(name) => name,
            JS::Int(i) => i.to_string(),
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
