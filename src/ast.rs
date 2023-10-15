use std::hint::unreachable_unchecked;

use crate::tokens::Token;
#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Var {
        name: Token,
    },
}
#[derive(Debug)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
    fn visit_var_expr(&mut self, expr: &Expr) -> T;
    fn visit_assign_expr(&mut self, expr: &Expr) -> T;
    fn visit_logical_expr(&mut self, expr: &Expr) -> T;
}
pub trait StmtVisitor<T> {
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> T;
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
            Expr::Logical {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_logical_expr(self),
            Expr::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary_expr(self),
            Expr::Var { name: _ } => visitor.visit_var_expr(self),
            Expr::Assign { name: _, value: _ } => visitor.visit_assign_expr(self),
        }
    }
}
impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression { expression: _ } => visitor.visit_expr_stmt(self),
            Stmt::Print { expression: _ } => visitor.visit_print_stmt(self),
            Stmt::Var {
                name: _,
                initializer: _,
            } => visitor.visit_var_stmt(self),
            Stmt::Block { statements: _ } => visitor.visit_block_stmt(self),
            Stmt::If {
                condition: _,
                then_branch: _,
                else_branch: _,
            } => visitor.visit_if_stmt(self),
            Stmt::While {
                condition: _,
                body: _,
            } => visitor.visit_while_stmt(self),
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
    fn visit_var_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Var { name } => format!("{:?}", name),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Assign { name, value } => format!("{:?}={:?}", name, value),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Logical {
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
}
