//! Module `infer` provides a very valuable utility `pub fn infer`.
//! It implements the infamous Hindley-Milner type inference algorithm that
//! operates on `infer::Expr`essions.
//!
//! This implementation is a port of
//! [this Hindley-Milner analyser](https://github.com/kritzcreek/fby19/tree/master)
//! (originally written in Haskell by
//! [Christoph Hegemann](https://github.com/kritzcreek))
//! who was kind enough to record a video presentation/explanation and post it
//! on [YouTube](https://youtu.be/ytPAlhnAKro?si=psgdXDTzEp3yQP0r).
//!
//! Some other useful resources include:
//!
//! - https://github.com/lorepozo/polytype-rs
//! - https://cs.brown.edu/~sk/Publications/Books/ProgLangs/2007-04-26/plai-2007-04-26.pdf
//! - https://eli.thegreenplace.net/2018/unification/
//! - https://cs.brown.edu/courses/cs173/2012/book/types.html

use crate::types::*;
use im::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unit,                       // ()
    Int(i32),                   // 42
    Name(Binder),               // x
    Call(Box<Self>, Box<Self>), // e e
    Func(Binder, Box<Self>),    // a -> b -> Expr
}

impl Expr {
    pub fn call(f: Self, args: Vec<Self>) -> Self {
        args.into_iter()
            .fold(f, |prev, next| Self::Call(prev.into(), next.into()))
    }
}

#[derive(Default, Clone)]
pub struct Context(HashMap<Binder, Scheme>);

#[derive(Default, Clone)]
pub struct Substitution(HashMap<Binder, Type>);

impl Context {
    fn lookup(&self, binder: &Binder) -> Scheme {
        self.0
            .get(binder)
            .expect(&format!("Undefined binder: {}", binder))
            .clone()
    }

    fn update(&self, binder: Binder, scheme: Scheme) -> Self {
        Self(self.0.update(binder, scheme))
    }
}

impl<I> From<I> for Substitution
where
    I: Iterator<Item = (Binder, Type)>,
{
    fn from(value: I) -> Self {
        Self(value.collect())
    }
}

impl Substitution {
    fn unify(t1: &Type, t2: &Type) -> Substitution {
        println!("Trying to unify: {:?} & {:?}", t1, t2);
        match (t1, t2) {
            (Type::Unit, Type::Unit) | (Type::Int, Type::Int) => {
                Substitution::default()
            }
            (Type::Var(binder), t) | (t, Type::Var(binder)) => {
                Self::bind(binder.clone(), t.clone())
            }
            (Type::Func(arg1, res1), Type::Func(arg2, res2)) => {
                let s1 = Self::unify(arg1, arg2);
                let s2 = Self::unify(
                    &s1.apply(*res1.clone()),
                    &s1.apply(*res2.clone()),
                );
                s2.union(&s1)
            }
            (t1, t2) => panic!("Unification error: {:?} & {:?}", t1, t2),
        }
    }

    fn bind(binder: String, t: Type) -> Substitution {
        println!("Trying to bind: {} := {:?}", binder, t);
        if let Type::Var(name) = t.clone() {
            if name == binder {
                return Substitution::default();
            }
        }
        if t.free_type_vars().contains(&binder) {
            panic!("Occurs check failed");
        }
        Substitution(HashMap::unit(binder, t))
    }

    fn union(&self, other: &Substitution) -> Self {
        let new: HashMap<Binder, Type> = other
            .0
            .clone()
            .into_iter()
            .map(|(binder, t)| (binder, self.apply(t)))
            .collect();
        Self(new.union(self.0.clone()))
    }

    fn apply(&self, t: Type) -> Type {
        match t {
            Type::Var(binder) => self
                .0
                .get(&binder)
                .map_or(Type::Var(binder), |got| got.clone()),
            Type::Func(arg, res) => {
                Type::Func(self.apply(*arg).into(), self.apply(*res).into())
            }
            other => other,
        }
    }

    fn apply_scheme(&self, scheme: Scheme) -> Scheme {
        scheme.map_body(|body| {
            self.without(scheme.guards.iter().cloned()).apply(body)
        })
    }

    fn apply_context(&self, ctx: Context) -> Context {
        let new = ctx
            .0
            .into_iter()
            .map(|(binder, scheme)| (binder, self.apply_scheme(scheme)))
            .collect();
        Context(new)
    }

    fn without<KS>(&self, keys: KS) -> Self
    where
        KS: Iterator<Item = Binder>,
    {
        let mut map = self.0.clone();
        for key in keys {
            map = map.without(&key);
        }
        Self(map)
    }
}

#[derive(Default)]
struct Fresh {
    count: usize,
}

impl Fresh {
    fn next(&mut self) -> Type {
        self.count += 1;
        Type::Var(format!("t{}", self.count - 1))
    }

    fn take(&mut self, n: usize) -> Vec<Type> {
        (0..n).into_iter().map(|_| self.next()).collect()
    }
}

#[derive(Default)]
struct HindleyMilner {
    var: Fresh,
}

impl HindleyMilner {
    /// Scheme instantiation results in a concrete Type. To instantiate a
    /// Scheme, we must generate fresh type variables for each of its guards.
    fn instantiate(&mut self, scheme: Scheme) -> Type {
        let n = scheme.guards.len();
        let sub: Substitution =
            scheme.guards.into_iter().zip(self.var.take(n)).into();
        sub.apply(scheme.body)
    }

    fn infer(&mut self, ctx: Context, expr: Expr) -> (Substitution, Type) {
        match expr {
            Expr::Unit => (Substitution::default(), Type::Unit),
            Expr::Int(_) => (Substitution::default(), Type::Int),
            Expr::Name(binder) => (
                Substitution::default(),
                self.instantiate(ctx.lookup(&binder)),
            ),
            Expr::Call(f, arg) => self.infer_call(ctx, *f, *arg),
            Expr::Func(binder, body) => self.infer_func(ctx, binder, *body),
        }
    }

    fn infer_call(
        &mut self,
        ctx: Context,
        f: Expr,
        arg: Expr,
    ) -> (Substitution, Type) {
        let result_type = self.var.next();
        let (s1, f_type) = self.infer(ctx.clone(), f);
        let (s2, arg_type) = self.infer(s1.apply_context(ctx), arg);
        let f1 = &s2.apply(f_type.clone());
        let f2 = &Type::Func(arg_type.into(), result_type.clone().into());
        let s3 = Substitution::unify(f1, f2);
        (s3.union(&s2).union(&s1), s3.apply(result_type))
    }

    fn infer_func(
        &mut self,
        ctx: Context,
        binder: Binder,
        body: Expr,
    ) -> (Substitution, Type) {
        let binder_type = self.var.next();
        let ctx_ = ctx.update(binder, Scheme::concrete(binder_type.clone()));
        let (subs, body_type) = self.infer(ctx_, body);
        (
            subs.clone(),
            Type::Func(subs.apply(binder_type).into(), body_type.into()),
        )
    }
}

pub fn infer(ctx: Context, expr: Expr) -> Type {
    let (sub, t) = HindleyMilner::default().infer(ctx, expr);
    sub.apply(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let id = Scheme::forall(
            "a".to_string(),
            Type::Func(
                Type::Var("a".to_string()).into(),
                Type::Var("a".to_string()).into(),
            ),
        );
        let add =
            Scheme::concrete(Type::func(vec![Type::Int, Type::Int], Type::Int));
        let ctx = Context(
            HashMap::unit("id".to_string(), id).update("add".to_string(), add),
        );

        let expr = Expr::Call(
            Expr::Name("id".to_string()).into(),
            Expr::Int(42).into(),
        );
        let t = infer(ctx.clone(), expr);
        assert_eq!(t, Type::Int);

        let expr =
            Expr::call(Expr::Name("add".to_string()), vec![Expr::Int(1)]);
        let t = infer(ctx.clone(), expr);
        assert_eq!(t, Type::Func(Type::Int.into(), Type::Int.into()));
    }
}
