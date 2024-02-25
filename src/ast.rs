use crate::{parser, parser::Rule};
use pest::iterators::{Pair, Pairs};

#[derive(Default, Debug, PartialEq)]
pub struct AST {
    declarations: Vec<Declaration>,
}

impl TryFrom<Pairs<'_, Rule>> for AST {
    type Error = String;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let declarations = pairs
            .into_iter()
            .take_while(parser::is_not_eoi)
            .map(|pair| pair.into_inner().into())
            .collect();
        Ok(Self { declarations })
    }
}

impl From<Pairs<'_, Rule>> for Declaration {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut it = pairs.into_iter();
        let id = Expr::string(it.next().unwrap());
        let expr = it.next().unwrap().into();
        Self { id, expr }
    }
}

#[derive(Debug, PartialEq)]
struct Declaration {
    id: String,
    expr: Expr,
}

impl From<Pair<'_, Rule>> for Expr {
    fn from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::int => Self::int(pair),
            Rule::name => Self::name(pair),
            Rule::id => Self::id(pair),
            Rule::call => Self::call(pair.into_inner().into()),
            Rule::func => Self::func(pair.into_inner().into()),
            _ => unreachable!("Expr from Pair"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Int(i32),                          // -42
    Name(String),                      // x
    ID(String),                        // main.example
    Call(Box<Expr>, Box<Vec<Expr>>),   // f a main.b 42 (std.print 58)
    Func(Box<Vec<String>>, Box<Expr>), // a -> b -> Expr
}

impl Expr {
    pub fn int(pair: Pair<Rule>) -> Self {
        Self::Int(pair.as_str().parse().unwrap())
    }

    pub fn name(pair: Pair<Rule>) -> Self {
        Self::Name(Self::string(pair))
    }

    pub fn id(pair: Pair<Rule>) -> Self {
        Self::ID(Self::string(pair))
    }

    pub fn call(pairs: Pairs<Rule>) -> Self {
        let mut it = pairs.into_iter();
        let f = it.next().unwrap().into();
        let args = it.map(|pair| pair.into()).collect();
        Expr::Call(Box::new(f), Box::new(args))
    }

    pub fn func(pairs: Pairs<Rule>) -> Self {
        let params = pairs
            .clone()
            .into_iter()
            .take_while(parser::is_param)
            .map(Expr::param)
            .collect();
        let expr = pairs
            .clone()
            .into_iter()
            .filter(|pair| !parser::is_param(pair))
            .next()
            .map(|expr| expr.into())
            .unwrap();
        Expr::Func(Box::new(params), Box::new(expr))
    }

    pub fn param(pair: Pair<Rule>) -> String {
        Self::string(pair.into_inner().next().unwrap())
    }

    pub fn string(pair: Pair<Rule>) -> String {
        pair.as_str().to_string()
    }
}
