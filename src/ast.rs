use std::hint::unreachable_unchecked;

use crate::tokens::Token;
#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
#[derive(Debug)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
}
pub trait StmtVisitor<T> {
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> T;
}
impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary_expr(self),
            Expr::Grouping { expression: _ } => visitor.visit_grouping_expr(self),
            Expr::Literal { value: _ } => visitor.visit_literal_expr(self),
            Expr::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary_expr(self),
        }
    }
}
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression { expression: _ } => visitor.visit_expr_stmt(self),
            Stmt::Print { expression: _ } => visitor.visit_print_stmt(self),
        }
    }
}
pub struct AstPrinter {}
impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}
impl ExprVisitor<String> for AstPrinter {
    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => format!("({:?}{})", operator, self.print(right)),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!(
                "({} {:?} {})",
                self.print(left),
                operator,
                self.print(right)
            ),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => format!("(group {})", self.print(expression)),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value } => format!("{:?}", value),
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
