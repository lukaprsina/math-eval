use serde::{Deserialize, Serialize};

use super::{product::Product, Expression, Node};
use std::{cmp::Ordering, collections::HashSet, ops::Mul};

pub(crate) trait ShouldBeParenthesized {
    fn should_be_parenthesized(&self) -> bool;
}

pub(crate) trait IsTimesVisible {
    fn is_times_visible(&self, last: &Element) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Sign {
    #[default]
    Positive,
    Negative,
}

impl Mul for Sign {
    type Output = Sign;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == rhs {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum NodeOrExpression {
    Node(Node),
    Expression(Expression),
}

impl Default for NodeOrExpression {
    fn default() -> Self {
        NodeOrExpression::Expression(Expression::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementCache {
    pub variables: HashSet<String>,
    pub functions: HashSet<String>,
    pub is_number: Option<bool>,
}

impl ElementCache {
    pub fn new() -> ElementCache {
        ElementCache {
            variables: HashSet::new(),
            functions: HashSet::new(),
            is_number: None,
        }
    }
}

impl PartialOrd for ElementCache {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl Ord for ElementCache {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default, Serialize, Deserialize)]
pub struct Element {
    pub sign: Sign,
    pub node_or_expression: NodeOrExpression,
    pub cache: Option<ElementCache>,
}

impl Element {
    pub fn new(sign: Sign, node_or_expression: NodeOrExpression) -> Self {
        Self {
            sign,
            node_or_expression,
            cache: None,
        }
    }

    // assume analyzed
    pub fn is_number(&self) -> bool {
        if self.cache.is_none() {
            panic!("Not analyzed");
        }

        match &self.node_or_expression {
            NodeOrExpression::Node(node) => {
                if let Node::Number(_) = node {
                    true
                } else {
                    false
                }
            }
            NodeOrExpression::Expression(expression) => {
                let mut is_number = true;

                'outer: for product in &expression.products {
                    for side in [&product.numerator, &product.denominator] {
                        for element in side {
                            if !element.is_number() {
                                is_number = false;
                                break 'outer;
                            }
                        }
                    }
                }

                is_number
            }
        }
    }

    pub fn invert_sign(&mut self) {
        match self.sign {
            Sign::Positive => self.sign = Sign::Negative,
            Sign::Negative => self.sign = Sign::Positive,
        }
    }
}

impl IsTimesVisible for Element {
    fn is_times_visible(&self, last: &Element) -> bool {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => node.is_times_visible(last),
            NodeOrExpression::Expression(expression) => expression.is_times_visible(last),
        }
    }
}

impl ShouldBeParenthesized for Element {
    fn should_be_parenthesized(&self) -> bool {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => node.should_be_parenthesized(),
            NodeOrExpression::Expression(expression) => expression.should_be_parenthesized(),
        }
    }
}

impl Element {
    pub fn simple_add(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![
            Product {
                numerator: vec![lhs],
                denominator: vec![],
            },
            Product {
                numerator: vec![rhs],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_sub(lhs: Element, mut rhs: Element) -> Element {
        rhs.invert_sign();
        let result = Expression::new(vec![
            Product {
                numerator: vec![lhs],
                denominator: vec![],
            },
            Product {
                numerator: vec![rhs],
                denominator: vec![],
            },
        ]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_mul(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![Product {
            numerator: vec![lhs, rhs],
            denominator: vec![],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_div(lhs: Element, rhs: Element) -> Element {
        let result = Expression::new(vec![Product {
            numerator: vec![lhs],
            denominator: vec![rhs],
        }]);

        Element::new(Sign::Positive, NodeOrExpression::Expression(result))
    }

    pub fn simple_neg(mut self) -> Element {
        self.invert_sign();
        self
    }

    pub fn simple_mul_sign(mut self, sign: Sign) -> Element {
        self.sign = self.sign * sign;
        self
    }
}
