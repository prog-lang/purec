use crate::stdlib;
use crate::valid::Valid;
use crate::{parser, parser::Rule};
use pest::iterators::{Pair, Pairs};
use std::collections::{HashMap, HashSet};

const ENTRYPOINT: &str = "main";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AST {
    pub declarations: HashMap<String, Declaration>,
}

impl TryFrom<Pairs<'_, Rule>> for AST {
    type Error = String;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let declarations = pairs
            .into_iter()
            .take_while(parser::is_not_eoi)
            .map(|pair| Declaration::from(pair.into_inner()))
            .map(|decl| (decl.id.clone(), decl))
            .collect();

        Self { declarations }.valid()
        // .map(AST::without_unused_declarations)
    }
}

impl Valid for AST {
    type Error = String;

    fn validate(&self) -> Result<(), Self::Error> {
        self.check_entrypoint_present()?;
        self.check_undef_ids()?;
        Ok(())
    }
}

impl AST {
    /// Declarations vector returned by this method is ordered such that the
    /// entrypoint is returned as the first element. There are no guarantees as
    /// to the ordering of the remaining declarations.
    pub fn get_declarations(&self) -> Vec<Declaration> {
        vec![self.declarations.get(ENTRYPOINT).unwrap()]
            .into_iter()
            .chain(
                self.declarations
                    .values()
                    .filter(|decl| decl.id != ENTRYPOINT),
            )
            .cloned()
            .collect()
    }

    pub fn get_declaration(&self, id: &String) -> Declaration {
        self.declarations.get(id).unwrap().clone()
    }

    #[allow(dead_code)]
    fn without_unused_declarations(mut self) -> Self {
        let declared: HashSet<String> =
            self.declarations.keys().cloned().collect();
        let referenced = &self.get_ref_ids();
        let unused = declared.difference(referenced);
        for reference in unused {
            self.declarations.remove(reference);
        }
        self
    }

    fn get_ref_ids(&self) -> HashSet<String> {
        self.declarations
            .iter()
            .map(|(_, decl)| decl.expr.get_ids())
            .flatten()
            .chain(vec![ENTRYPOINT.to_string()].into_iter())
            .collect()
    }

    fn get_known_ids(&self) -> HashSet<String> {
        self.declarations
            .keys()
            .chain(stdlib::index().keys())
            .cloned()
            .collect()
    }

    fn get_undef_ids(&self) -> Vec<String> {
        self.get_ref_ids()
            .difference(&self.get_known_ids())
            .cloned()
            .collect()
    }

    fn check_undef_ids(&self) -> Result<(), String> {
        let diff = self.get_undef_ids();
        if diff.is_empty() {
            Ok(())
        } else {
            Err(format!("Unknown references found: {}", diff.join(", ")))
        }
    }

    fn check_entrypoint_present(&self) -> Result<(), String> {
        match self.declarations.get(ENTRYPOINT) {
            None => Err(format!("Missing entrypoint: {}", ENTRYPOINT)),
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub id: String,
    pub expr: Expr,
}

impl From<Pairs<'_, Rule>> for Declaration {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut it = pairs.into_iter();
        let id = Expr::string(it.next().unwrap());
        let expr = it.next().unwrap().into();
        Self { id, expr }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
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
