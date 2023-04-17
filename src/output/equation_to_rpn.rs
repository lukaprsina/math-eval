use crate::ast::{product::Product, Element, Equation, Expression, Node, NodeOrExpression};

pub trait ReversePolishNotation {
    fn rpn(&self) -> String;
}

impl ReversePolishNotation for Equation {
    fn rpn(&self) -> String {
        let mut result = String::new();

        let len = self.equation_sides.len() as isize - 1;
        if len.is_positive() {
            for side in &self.equation_sides[0..len as usize] {
                let side_rpn = side.rpn();
                result += &format!("{side_rpn} = ",);
            }
        }

        if let Some(last) = self.equation_sides.last() {
            let side_rpn = last.rpn();
            result += &format!("{side_rpn}")
        }

        //  1  2  -  3 *
        let mut new_result = String::new();
        if let Some(mut previous_char) = result.trim().chars().nth(0) {
            if !previous_char.is_whitespace() {
                new_result.push(previous_char);
            }

            for character in result.chars().skip(1).into_iter() {
                if !previous_char.is_whitespace() || !character.is_whitespace() {
                    new_result.push(character);
                }

                previous_char = character;
            }
        }

        new_result
    }
}

impl ReversePolishNotation for Element {
    fn rpn(&self) -> String {
        match &self.node_or_expression {
            NodeOrExpression::Node(node) => node.rpn(),
            NodeOrExpression::Expression(expression) => expression.rpn(),
        }
    }
}

impl ReversePolishNotation for Expression {
    fn rpn(&self) -> String {
        if self.products.len() == 0 {
            return String::new();
        }

        let mut result = String::new();

        for (pos, product) in self.products.iter().enumerate() {
            if product.numerator.len() == 0 && product.denominator.len() == 0 {
                break;
            }

            let product_rpn = product.rpn();
            result.push_str(&format!("{product_rpn} "));

            if pos >= 1 {
                let sign = product.calculate_sign();
                result.push_str(&format!("{sign} "));
            }
        }

        result
    }
}

impl ReversePolishNotation for Product {
    fn rpn(&self) -> String {
        let mut result = String::new();

        for (side_pos, side) in [&self.numerator, &self.denominator].into_iter().enumerate() {
            if side_pos == 0 && self.numerator.is_empty() {
                result += "1l ";
            }

            for (elem_pos, element) in side.into_iter().enumerate() {
                let element_rpn = element.rpn();
                result.push_str(&format!("{element_rpn} "));

                if elem_pos >= 1 {
                    result.push_str("* ");
                }
            }

            if !self.denominator.is_empty() {
                result.push(' ');
            }
        }

        if !self.denominator.is_empty() {
            result.push('/');
        }

        result
    }
}

impl ReversePolishNotation for Node {
    fn rpn(&self) -> String {
        match self {
            Node::Number(number) => number.to_string(),
            Node::Variable(name) => name.to_string(),
            Node::Power { base, power } => format!("{base} {power} ^"),
            Node::Modulo { lhs, rhs } => format!("{lhs} {rhs} %"),
            Node::Factorial { child } => format!("{child}!"),
            Node::Function { name, arguments } => {
                let mut result = format!("{name}(");

                for (index, argument) in arguments.iter().enumerate() {
                    result += &argument.rpn();
                    if index < arguments.len() - 1 {
                        result += ", ";
                    }
                }

                result.push(')');
                result
            }
        }
    }
}
