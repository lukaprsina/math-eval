use crate::tokenizer::Operation;

use super::{Element, NodeOrExpression};

#[derive(Debug, Clone)]
pub struct Equation {
    pub sides: Vec<EquationSide>,
}

#[derive(Debug, Clone)]
pub struct EquationSide {
    pub element: Element,
    pub operation: Option<Operation>,
}

impl EquationSide {
    pub fn new(element: Element, operation: Option<Operation>) -> Self {
        Self { element, operation }
    }
}

impl Equation {
    pub fn flatten(&mut self) {
        for side in self.sides.iter_mut() {
            if let NodeOrExpression::Expression(expression) = &mut side.element.node_or_expression {
                expression.flatten();
            }
        }
    }
}
