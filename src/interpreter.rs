use std::{hint::unreachable_unchecked, mem};

use thiserror::Error;

use crate::{
    ast::{Expr, ExprVisitor, Stmt, StmtVisitor},
    environment::Environment,
    tokens::TokenType,
};
#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("{0}\n[line {1}]")]
    RuntimeError(String, usize),
}
type InterpreterResult = Result<TokenType, InterpreterError>;
#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }
    pub fn interpret(&mut self, stmt: Vec<Stmt>) -> InterpreterResult {
        for stmt in stmt {
            self.execute(&stmt)?;
        }
        Ok(TokenType::Nil)
    }
    fn execute(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        stmt.accept(self)
    }
    fn evaluate(&mut self, expr: &Expr) -> InterpreterResult {
        expr.accept(self)
    }
    fn get_number(tok: TokenType, err_msg: String) -> Result<f64, InterpreterError> {
        match tok {
            TokenType::Number(n) => Ok(n),
            _ => Err(InterpreterError::RuntimeError(err_msg, 0)),
        }
    }
    fn get_numbers(t1: TokenType, t2: TokenType) -> Result<(f64, f64), InterpreterError> {
        let n1 = Interpreter::get_number(t1, "Left operand must be a number.".to_string())?;
        let n2 = Interpreter::get_number(t2, "Right operand must be a number.".to_string())?;
        Ok((n1, n2))
    }
    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), InterpreterError> {
        let previous = mem::replace(&mut self.environment, environment);
        for stmt in statements {
            self.execute(stmt)?;
        }
        self.environment = previous;
        Ok(())
    }
}
impl StmtVisitor<Result<(), InterpreterError>> for Interpreter {
    fn visit_print_stmt(&mut self, stmt: &crate::ast::Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if TryInto::<bool>::try_into(self.evaluate(condition)?).unwrap() {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_expr_stmt(&mut self, stmt: &crate::ast::Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_var_stmt(&mut self, stmt: &crate::ast::Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.define(name.to_string(), value);
                Ok(())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::Block { statements } => self.execute_block(
                statements,
                Environment::new(Some(Box::new(self.environment.clone()))),
            ),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), InterpreterError> {
        match stmt {
            Stmt::While { condition, body } => {
                while TryInto::<bool>::try_into(self.evaluate(condition)?).unwrap() {
                    self.execute(body)?;
                }
                Ok(())
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
impl ExprVisitor<Result<TokenType, InterpreterError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> InterpreterResult {
        let (left, operator, right) = match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => (left, operator, right),
            _ => unsafe { unreachable_unchecked() },
        };
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok(TokenType::Number(nl - nr))
            }
            TokenType::Slash => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok(TokenType::Number(nl / nr))
            }
            TokenType::Star => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok(TokenType::Number(nl * nr))
            }
            TokenType::Plus => {
                if let (TokenType::Number(l), TokenType::Number(r)) = (left.clone(), right.clone())
                {
                    Ok(TokenType::Number(l + r))
                } else if let (TokenType::String(l), TokenType::String(r)) =
                    (left.clone(), right.clone())
                {
                    Ok(TokenType::String(l + &r))
                } else {
                    Err(InterpreterError::RuntimeError(
                        "Cant add these stuff".to_string(),
                        operator.line,
                    ))
                }
            }
            TokenType::Greater => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok((nl > nr).try_into().unwrap())
            }
            TokenType::GreaterEqual => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok((nl >= nr).try_into().unwrap())
            }
            TokenType::Less => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok((nl < nr).try_into().unwrap())
            }
            TokenType::LessEqual => {
                let (nl, nr) = Interpreter::get_numbers(left, right)?;
                Ok((nl <= nr).try_into().unwrap())
            }
            TokenType::BangEqual => Ok((left != right).try_into().unwrap()),
            TokenType::EqualEqual => Ok((left == right).try_into().unwrap()),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> InterpreterResult {
        match expr {
            Expr::Grouping { expression } => self.evaluate(expression),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_literal_expr(&mut self, expr: &crate::ast::Expr) -> InterpreterResult {
        match expr {
            Expr::Literal { value } => Ok(value.token_type.clone()),
            _ => unsafe { unreachable_unchecked() },
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> InterpreterResult {
        match expr {
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        let num = Interpreter::get_number(
                            right,
                            "Operand must be a number.".to_string(),
                        )?;
                        Ok(TokenType::Number(-num))
                    }
                    TokenType::Bang => Ok((!TryInto::<bool>::try_into(right).unwrap())
                        .try_into()
                        .unwrap()),
                    _ => unsafe { unreachable_unchecked() },
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_var_expr(&mut self, expr: &Expr) -> Result<TokenType, InterpreterError> {
        match expr {
            Expr::Var { name } => match self.environment.get(name.to_string().as_str()) {
                Some(v) => Ok(v.clone()),
                None => Err(InterpreterError::RuntimeError(
                    "Undefined variable".to_string(),
                    0,
                )),
            },
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<TokenType, InterpreterError> {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment
                    .assign(name.to_string().as_str(), value.clone())
                    .ok_or_else(|| {
                        InterpreterError::RuntimeError("Undefined variable".to_string(), name.line)
                    })?;
                Ok(value)
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<TokenType, InterpreterError> {
        match expr {
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                match operator.token_type {
                    TokenType::Or => {
                        if TryInto::<bool>::try_into(left.clone()).unwrap() {
                            Ok(left)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    TokenType::And => {
                        if !TryInto::<bool>::try_into(left.clone()).unwrap() {
                            Ok(left)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    _ => unsafe { unreachable_unchecked() },
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
