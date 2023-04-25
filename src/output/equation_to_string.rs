use crate::ast::{Equation, Expression, Node, Sign};
use std::fmt::{Display, Write};

use crate::ast::{element::ShouldBeParenthesized, product::Product, Element, NodeOrExpression};

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        let len = self.equation_sides.len() as isize - 1;
        if len.is_positive() {
            for side in &self.equation_sides[0..len as usize] {
                result += &format!("{} = ", side)
            }
        }

        if let Some(last) = self.equation_sides.last() {
            result += &format!("{}", last)
        }

        write!(f, "{}", result)
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Positive => write!(f, "+"),
            Sign::Negative => write!(f, "-"),
        }
    }
}

impl Display for NodeOrExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            NodeOrExpression::Node(node) => node.to_string(),
            NodeOrExpression::Expression(expression) => expression.to_string(),
        };

        write!(f, "{}", result)
    }
}

fn to_string_with_parenthesis<T: ShouldBeParenthesized + Display>(item: &T) -> String {
    let parenthesis = item.should_be_parenthesized();
    format!(
        "{}{}{}",
        if parenthesis { "(" } else { "" },
        &item.to_string(),
        if parenthesis { ")" } else { "" },
    )
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        match self {
            Node::Number(number) => result += &number.to_string(),
            Node::Variable(variable) => result += variable,
            Node::Power { base, power } => {
                write!(
                    result,
                    "{}^{}",
                    to_string_with_parenthesis(base.as_ref()),
                    to_string_with_parenthesis(power.as_ref()),
                )
                .unwrap();
            }
            Node::Modulo { lhs, rhs } => {
                write!(
                    result,
                    "{}%{}",
                    to_string_with_parenthesis(lhs.as_ref()),
                    to_string_with_parenthesis(rhs.as_ref())
                )
                .unwrap();
            }
            Node::Factorial { child } => {
                write!(result, "{}!", to_string_with_parenthesis(child.as_ref())).unwrap();
            }
            Node::Function { name, arguments } => {
                write!(result, "{}(", name).unwrap();
                for (index, argument) in arguments.iter().enumerate() {
                    result += &argument.to_string();
                    if index < arguments.len() - 1 {
                        result += ", ";
                    }
                }
                result += ")";
            }
        }

        write!(f, "{}", result)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for product in &self.products {
            result += &product.to_string();
        }

        write!(f, "{}", result)
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        if self.numerator.is_empty() {
            result += " 1";
        }

        // println!("{self:#?}");

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            let open = side.should_be_parenthesized();

            if open {
                result.push('(');
            }

            for (element_pos, element) in side.iter().enumerate() {
                let element_open = (element_pos > 0 || (side_pos == 1 && element_pos == 0))
                    && element.sign == Sign::Negative;

                if element_open {
                    result.push('(');
                }

                result += &format!("{} {} ", element.sign, element.to_string());

                if element_open {
                    // result.push_str(&format!("_!{element_pos}_"));
                    result.push(')');
                }

                // TODO: -6 isn't in denominator
                // 1/((((-6)+1)+1)+1)
                /* if side.len() > 1 && element_pos == 0 {
                    result.push_str(&format!("__:{side_pos}:{element_pos}:{}", side.len()));
                } */
            }

            if open {
                result.push(')');
            }

            if side_pos == 0 && !self.denominator.is_empty() {
                result.push('/');
            }
        }

        write!(f, "{}", result)
    }
}

/* TODO:
5-6
5*(-6)
 */

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        // println!("{self:#?}");

        let one_product = match &self.node_or_expression {
            NodeOrExpression::Expression(expression) => expression.products.len() < 2,
            NodeOrExpression::Node(_) => true,
        };

        let open_element = matches!(self.node_or_expression, NodeOrExpression::Expression(_))
            && (self.sign == Sign::Negative || !one_product);
        if open_element {
            result.push('(');
        }

        match &self.node_or_expression {
            NodeOrExpression::Expression(expression) => {
                for product in &expression.products {
                    result.push_str(&product.to_string());
                }
            }
            NodeOrExpression::Node(node) => result += &node.to_string(),
        }

        if open_element {
            result.push(')');
        }

        // println!("{result}");
        write!(f, "{}", result)
    }
}
