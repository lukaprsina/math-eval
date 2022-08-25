use std::collections::HashMap;

use crate::ast::{product::Product, Element, Equation, Expression, Node, NodeOrExpression};

pub trait IsSame {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool;
}

#[derive(Debug)]
pub struct IsSameNames {
    pub variables: HashMap<String, Vec<String>>,
    pub functions: HashMap<String, Vec<String>>,
}

impl IsSameNames {
    pub fn new() -> IsSameNames {
        IsSameNames {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

impl<T: Ord + Clone + IsSame> IsSame for Vec<T> {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        if lhs.len() != rhs.len() {
            return false;
        }

        if lhs.is_empty() || rhs.is_empty() {
            return true;
        }

        let mut a = lhs.clone();
        let mut b = rhs.clone();
        a.sort();
        b.sort();

        let mut result = false;
        for left in a.iter() {
            for right in b.iter() {
                let are_same = T::is_same(&left, &right, names);
                result |= are_same;
                if result {
                    break;
                }
            }
        }

        result
    }
}

impl IsSame for Equation {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        if lhs.eq_sides.len() != rhs.eq_sides.len() {
            return false;
        }
        // TODO: not true
        let mut result = true;

        for (left, right) in lhs.eq_sides.iter().zip(&rhs.eq_sides) {
            result &= Element::is_same(left, right, names);
            if !result {
                break;
            }
        }

        result
    }
}

impl IsSame for Element {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        lhs.sign == rhs.sign
            && NodeOrExpression::is_same(&lhs.node_or_expression, &rhs.node_or_expression, names)
    }
}

impl IsSame for NodeOrExpression {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        match lhs {
            NodeOrExpression::Node(l_node) => match rhs {
                NodeOrExpression::Node(r_node) => Node::is_same(l_node, r_node, names),
                NodeOrExpression::Expression(_) => false,
            },
            NodeOrExpression::Expression(l_expr) => match rhs {
                NodeOrExpression::Node(_) => false,
                NodeOrExpression::Expression(r_expr) => Expression::is_same(l_expr, r_expr, names),
            },
        }
    }
}

impl IsSame for Node {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        match lhs {
            Node::Number(left_number) => {
                if let Node::Number(right_number) = rhs {
                    left_number == right_number
                } else {
                    false
                }
            }
            Node::Variable(left_name) => {
                if let Node::Variable(right_name) = rhs {
                    match names.variables.get_mut(left_name) {
                        Some(name) => {
                            name.push(right_name.clone());
                        }
                        None => {
                            names
                                .variables
                                .insert(left_name.clone(), vec![right_name.clone()]);
                        }
                    }
                }

                true
            }
            Node::Unit(left_name) => {
                if let Node::Unit(right_name) = rhs {
                    left_name == right_name
                } else {
                    false
                }
            }
            Node::Power {
                base: left_base,
                power: left_power,
            } => {
                if let Node::Power {
                    base: right_base,
                    power: right_power,
                } = rhs
                {
                    Element::is_same(left_base, right_base, names)
                        && Element::is_same(left_power, right_power, names)
                } else {
                    false
                }
            }
            Node::Modulo {
                lhs: left_lhs,
                rhs: left_rhs,
            } => {
                if let Node::Modulo {
                    lhs: right_lhs,
                    rhs: right_rhs,
                } = rhs
                {
                    Element::is_same(left_lhs, right_lhs, names)
                        && Element::is_same(left_rhs, right_rhs, names)
                } else {
                    false
                }
            }
            Node::Factorial { child: left_child } => {
                if let Node::Factorial { child: right_child } = rhs {
                    Element::is_same(left_child, right_child, names)
                } else {
                    false
                }
            }
            Node::Function {
                name: left_name,
                arguments: left_arguments,
            } => {
                if let Node::Function {
                    name: right_name,
                    arguments: right_arguments,
                } = rhs
                {
                    if Vec::is_same(left_arguments, right_arguments, names) {
                        match names.functions.get_mut(left_name) {
                            Some(name) => {
                                name.push(right_name.clone());
                            }
                            None => {
                                names
                                    .functions
                                    .insert(left_name.clone(), vec![right_name.clone()]);
                            }
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }

                true
                /* if let Node::Function {
                    name: right_name,
                    arguments: right_arguments,
                } = rhs
                {
                    left_name == right_name && Vec::is_same(left_arguments, right_arguments, names)
                } else {
                    false
                } */
            }
        }
    }
}

impl IsSame for Expression {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        let result = Vec::is_same(&lhs.products, &rhs.products, names);
        result
    }
}

impl IsSame for Product {
    fn is_same(lhs: &Self, rhs: &Self, names: &mut IsSameNames) -> bool {
        let mut result = Vec::is_same(&lhs.numerator, &rhs.numerator, names);
        result &= Vec::is_same(&lhs.denominator, &rhs.denominator, names);
        result
    }
}
