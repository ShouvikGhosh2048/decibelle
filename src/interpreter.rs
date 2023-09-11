use std::collections::HashMap;

use crate::{
    parser::{
        BinaryOperation, Expression, ExpressionType, Statement, StatementType, UnaryOperation,
    },
    tokenizer::Value,
};

struct Variables<'a> {
    global_variables: &'a mut HashMap<String, Value>,
    environments: Vec<HashMap<String, Value>>,
}

impl<'a> Variables<'a> {
    fn push_environment(&mut self) {
        self.environments.push(HashMap::new());
    }

    fn pop_environment(&mut self) {
        self.environments.pop();
    }

    fn get_variable(&self, variable: &String) -> Option<Value> {
        for environment in self.environments.iter().rev() {
            if let Some(value) = environment.get(variable) {
                return Some(value.clone());
            }
        }
        self.global_variables.get(variable).cloned()
    }

    /// Sets a variable. Doesn't create a new one.
    fn set_variable(&mut self, variable: &String, value: Value) -> Result<(), ()> {
        for environment in self.environments.iter_mut().rev() {
            if environment.contains_key(variable) {
                environment.insert(variable.clone(), value);
                return Ok(());
            }
        }

        if self.global_variables.contains_key(variable) {
            self.global_variables.insert(variable.clone(), value);
            Ok(())
        } else {
            Err(())
        }
    }

    fn create_variable(&mut self, variable: &str, value: Value) {
        if let Some(environment) = self.environments.last_mut() {
            environment.insert(variable.to_owned(), value);
        } else {
            self.global_variables.insert(variable.to_owned(), value);
        }
    }
}

pub fn interpret(statements: &Vec<Statement>, global_variables: &mut HashMap<String, Value>) {
    let mut variables = Variables {
        global_variables,
        environments: vec![],
    };
    for statement in statements {
        interpret_statement(statement, &mut variables);
    }
}

fn interpret_statement(statement: &Statement, variables: &mut Variables) {
    match &statement.statement {
        StatementType::Expression(expression) => {
            println!("{}", interpret_expression(expression, variables));
        }
        StatementType::VariableDeclaration {
            variable, value, ..
        } => {
            let value = interpret_expression(value, variables);
            variables.create_variable(variable, value);
        }
        StatementType::Block(statements) => {
            variables.push_environment();
            for statement in statements {
                interpret_statement(statement, variables);
            }
            variables.pop_environment();
        }
        StatementType::If {
            expression,
            then_statement,
            else_statement,
        } => {
            let Value::Boolean(value) = interpret_expression(expression, variables) else {
                unreachable!();
            };

            if value {
                interpret_statement(then_statement, variables);
            } else if let Some(else_statement) = else_statement {
                interpret_statement(else_statement, variables);
            }
        }
        StatementType::While {
            expression,
            statement,
        } => loop {
            let Value::Boolean(run_loop) = interpret_expression(expression, variables) else {
                    unreachable!();
                };

            if run_loop {
                interpret_statement(statement, variables);
            } else {
                break;
            }
        },
    }
}

fn interpret_expression(expression: &Expression, variables: &mut Variables) -> Value {
    match &expression.expression_type {
        ExpressionType::Unary {
            operation,
            expression,
        } => match operation {
            UnaryOperation::Minus => {
                let expression_value = interpret_expression(expression, variables);
                if let Value::Number(number) = expression_value {
                    Value::Number(-number)
                } else {
                    unreachable!()
                }
            }
            UnaryOperation::Not => {
                let expression_value = interpret_expression(expression, variables);
                if let Value::Boolean(boolean) = expression_value {
                    Value::Boolean(!boolean)
                } else {
                    unreachable!()
                }
            }
        },
        ExpressionType::Binary {
            operation,
            left_expression,
            right_expression,
        } => match operation {
            BinaryOperation::Add => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                    (Value::String(left), Value::String(right)) => Value::String(left + &right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Subtract => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Multiply => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Divide => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Number(left / right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Equal => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left == right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left == right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left == right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::NotEqual => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left != right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left != right),
                    (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left != right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Less => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left < right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left < right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::LessEqual => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left <= right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left <= right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Greater => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left > right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left > right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::GreaterEqual => {
                let left_value = interpret_expression(left_expression, variables);
                let right_value = interpret_expression(right_expression, variables);
                match (left_value, right_value) {
                    (Value::Number(left), Value::Number(right)) => Value::Boolean(left >= right),
                    (Value::String(left), Value::String(right)) => Value::Boolean(left >= right),
                    _ => {
                        unreachable!()
                    }
                }
            }
            BinaryOperation::Or => {
                let left_value = interpret_expression(left_expression, variables);
                if left_value == Value::Boolean(true) {
                    left_value
                } else {
                    interpret_expression(right_expression, variables)
                }
            }
            BinaryOperation::And => {
                let left_value = interpret_expression(left_expression, variables);
                if left_value == Value::Boolean(false) {
                    left_value
                } else {
                    interpret_expression(right_expression, variables)
                }
            }
            BinaryOperation::Assignment => {
                let value = interpret_expression(right_expression, variables);
                match &left_expression.expression_type {
                    ExpressionType::Variable(variable) => {
                        variables.set_variable(variable, value.clone()).unwrap();
                        value
                    }
                    ExpressionType::TupleAccess { expression, index } => {
                        // We access the tuple field and mutate it if the field is in a variable.
                        let mut current_expression = expression;
                        let mut indices = vec![*index];
                        loop {
                            match &current_expression.expression_type {
                                ExpressionType::TupleAccess { expression, index } => {
                                    current_expression = expression;
                                    indices.push(*index);
                                }
                                ExpressionType::Variable(variable) => {
                                    let mut variable_value =
                                        variables.get_variable(variable).unwrap();
                                    let mut lvalue = &mut variable_value;
                                    for &index in indices.iter().rev() {
                                        let Value::Tuple(values) = lvalue else { unreachable!() };
                                        lvalue = &mut values[index];
                                    }
                                    *lvalue = value.clone();
                                    variables.set_variable(variable, variable_value).unwrap();
                                    break;
                                }
                                _ => {
                                    // NOTE: may change when I add pointers.
                                    interpret_expression(expression, variables);
                                    break;
                                }
                            }
                        }
                        value
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        },
        ExpressionType::Variable(variable) => variables.get_variable(variable).unwrap(), // TODO: Handle this gracefully.
        ExpressionType::Literal(value) => value.clone(),
        ExpressionType::Grouping(expression) => interpret_expression(expression, variables),
        ExpressionType::Tuple(expressions) => Value::Tuple(
            expressions
                .iter()
                .map(|expression| interpret_expression(expression, variables))
                .collect(),
        ),
        ExpressionType::TupleAccess { expression, index } => {
            let Value::Tuple(values) = interpret_expression(expression, variables) else {
                unreachable!()
            };
            values[*index].clone()
        }
    }
}
