mod ast {
    pub enum Expr<T> {
        Lit(T),
        Add(Box<Expr<T>>, Box<Expr<T>>),
        Sub(Box<Expr<T>>, Box<Expr<T>>),
    }
}

mod visit {
    use crate::ast::*;

    pub trait Visitor<T> {
        fn visit_expr(&mut self, e: &Expr<T>) -> T;
    }
}

use std::ops::{Add, Sub};

use ast::*;
use visit::*;

struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_expr(&mut self, e: &Expr<i64>) -> i64 {
        match *e {
            Expr::Lit(n) => n,
            Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

fn dynamic_visit<T: Add<Output = T> + Sub<Output = T>>(v: &mut dyn Visitor<T>, e: Expr<T>) -> T {
    match e {
        Expr::Lit(n) => n,
        Expr::Add(ref lhs, ref rhs) => v.visit_expr(lhs) + v.visit_expr(rhs),
        Expr::Sub(ref lhs, ref rhs) => v.visit_expr(lhs) - v.visit_expr(rhs),
    }
}

fn main() {
    let mut int = Interpreter {};
    let sum = Expr::Add(Box::new(Expr::Lit(10)), Box::new(Expr::Lit(10)));
    let sub = Expr::Sub(Box::new(Expr::Lit(10)), Box::new(Expr::Lit(5)));
    assert!(int.visit_expr(&sum) == 20);
    assert!(int.visit_expr(&sub) == 5);

    let dynamic_sym = dynamic_visit(&mut int, sum);
    assert!(dynamic_sym == 20);
}
