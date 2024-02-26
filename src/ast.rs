use crate::stdlib;
use crate::valid::Valid;
use crate::{parser, parser::Rule};
use pest::iterators::{Pair, Pairs};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct AST {
    declarations: Vec<Declaration>,
    index: HashMap<String, usize>,
}

impl AST {
    fn get_entrypoint_index(&self) -> Option<usize> {
        self.index.get("main.main").map(|i| *i)
    }

    fn get_ref_ids(&self) -> HashSet<String> {
        self.declarations
            .iter()
            .map(|d| d.expr.get_ids())
            .flatten()
            .collect()
    }

    fn get_known_ids(&self) -> HashSet<String> {
        self.index.keys().cloned().collect()
    }

    fn get_undef_ids(&self) -> Vec<String> {
        self.get_ref_ids()
            .difference(&self.get_known_ids())
            .cloned()
            .collect()
    }
}

impl TryFrom<Pairs<'_, Rule>> for AST {
    type Error = String;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let declarations: Vec<Declaration> = pairs
            .into_iter()
            .take_while(parser::is_not_eoi)
            .map(|pair| pair.into_inner().into())
            .collect();

        let index = stdlib::index()
            .into_iter()
            .chain(
                declarations
                    .iter()
                    .enumerate()
                    .map(|(i, d)| (d.id.clone(), i)),
            )
            .collect();

        Self {
            declarations,
            index,
        }
        .valid()
    }
}

impl Valid for AST {
    type Error = String;

    fn validate(&self) -> Result<(), Self::Error> {
        let diff = self.get_undef_ids();
        if diff.is_empty() {
            Ok(())
        } else {
            Err(format!("Unknown references found: {}", diff.join(", ")))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Declaration {
    id: String,
    expr: Expr,
}

impl From<Pairs<'_, Rule>> for Declaration {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut it = pairs.into_iter();
        let id = Expr::string(it.next().unwrap());
        let expr = Expr::from(it.next().unwrap()).to_function();
        Self { id, expr }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Int(i32),                          // -42
    Name(String),                      // x
    ID(String),                        // main.example
    Call(Box<Self>, Box<Vec<Self>>),   // f a main.b 42 (std.print 58)
    Func(Box<Vec<String>>, Box<Self>), // a -> b -> Expr
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

impl Expr {
    fn to_function(self) -> Self {
        match self {
            Self::Func(params, expr) => Self::Func(params, expr),
            other => Self::Func(Box::new(vec![]), Box::new(other)),
        }
    }

    fn get_ids(&self) -> HashSet<String> {
        match self {
            Self::ID(id) => HashSet::from([id.clone()]),
            Self::Call(f, args) => f
                .get_ids()
                .into_iter()
                .chain(args.iter().map(|arg| arg.get_ids()).flatten())
                .collect(),
            Self::Func(_, expr) => expr.get_ids(),
            _ => HashSet::new(),
        }
    }

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
        Self::Call(Box::new(f), Box::new(args))
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
