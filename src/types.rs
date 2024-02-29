use crate::ast::Expr;
use crate::stdlib;
use polytype::{ptp, tp, Context, Type, UnificationError};

#[derive(Default)]
struct Checker {
    ctx: Context,
}

impl Checker {
    fn check(&mut self, expr: Expr, expect: Type) -> Result<(), UnificationError<&'static str>> {
        match expr {
            Expr::Int(_) => self.ctx.unify(&expect, &tp!(Int)),
            Expr::Name(_) => todo!(),
            Expr::ID(id) => {
                let got = stdlib::function(&id).unwrap().t.instantiate(&mut self.ctx);
                self.ctx.unify(&expect, &got)
            }
            Expr::Call(f, args) => {
                let mut tf: Type = args
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Type::Variable(i))
                    .rev()
                    .fold(expect, |beta, alpha| Type::arrow(alpha, beta));
                self.check(*f, tf)?;
                Ok(())
            }
            Expr::Func(_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converter() {
        let expect = tp!(Result);
        let t = (0..3)
            .into_iter()
            .enumerate()
            .map(|(i, _)| Type::Variable(i))
            .rev()
            .fold(expect, |prev, cur| Type::arrow(cur, prev));
        println!("{}", t.to_string());
    }

    #[test]
    fn it_works() {
        Checker::default()
            .check(Expr::Int(42), tp!(Int))
            .expect("int");
        Checker::default()
            .check(
                Expr::ID("std.add".to_string()),
                tp!(@arrow[tp!(Int), tp!(Int), tp!(Int)]),
            )
            .expect("id");
        Checker::default()
            .check(
                Expr::Call(
                    Box::new(Expr::ID("std.id".to_string())),
                    Box::new(vec![Expr::Int(42)]),
                ),
                tp!(Int),
            )
            .expect("call")
    }
}
