use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::ast::{product::Product, Element, Equation, Expression, Node, NodeOrExpression, Sign};

use super::strategy::Strategy;

#[derive(Debug, Clone)]
pub enum EquationTransformation {
    Function(String),
    InverseFunction(String),
    Multiply {
        multiply: Product,
        side_with_variable: Product,
    },
    Add {
        add: Expression,
        side_with_variable: Expression,
    },
}

#[derive(Debug, Clone)]
struct TransformResult {
    pub transformation: EquationTransformation,
    pub constraints: Vec<String>,
}

// imply that it has been analysed
fn apply_inverse(equation: &mut Equation) -> Vec<String> {
    if equation.equation_sides.len() != 2 {
        return vec![];
    }

    let mut constraints = vec![];

    let mut inverse = None;

    for side_element in &mut equation.equation_sides {
        match &mut side_element.cache {
            Some(cache) => {
                if cache.variables.len() >= 1 {
                    inverse = get_element_inverse(side_element);
                    // debug!("{:#?}", inverse);
                    break;
                }
            }
            None => panic!("Equation has not been analyzed, cannot simplify"),
        }
    }

    if let Some(transform_result) = inverse {
        transform_equation(equation, transform_result.transformation);

        constraints.extend(transform_result.constraints);
    }

    constraints
}

pub fn get_apply_inverse() -> Strategy {
    Strategy {
        apply: Some(Box::new(apply_inverse)),
        check: None,
    }
}

static INVERSE_FUNCTIONS: Lazy<HashMap<String, (String, Vec<String>)>> = Lazy::new(|| {
    let map: HashMap<&str, (&str, Vec<&str>)> = HashMap::from([
        ("sin", ("arcsin", vec![])),
        ("cos", ("arccos", vec![])),
        ("tan", ("arctan", vec![])),
        ("cot", ("arccot", vec![])),
    ]);

    let mut new_map: HashMap<String, (String, Vec<String>)> = HashMap::new();
    for (key, value) in map.into_iter() {
        let new_key = key.to_string();

        let new_value = (
            value.0.to_string(),
            value
                .1
                .into_iter()
                .map(|constraint| constraint.to_string())
                .collect_vec(),
        );

        new_map.insert(new_key, new_value);
    }
    new_map
});

fn get_element_inverse(element: &Element) -> Option<TransformResult> {
    let mut constraints: Vec<String> = vec![];

    let inverse = match &element.node_or_expression {
        NodeOrExpression::Node(node) => match node {
            Node::Power { base, power } => {
                if let (Some(b_cache), Some(p_cache)) = (&base.cache, &power.cache) {
                    if b_cache.variables.len() == 1 {
                        None
                    } else if p_cache.variables.len() == 1 {
                        None
                    } else {
                        None
                    }
                } else {
                    panic!("Not analyzed when getting the inverse")
                }
            }
            Node::Function { name, arguments: _ } => {
                if let Some(value) = INVERSE_FUNCTIONS.get(name) {
                    constraints.extend(value.1.clone());
                    Some(EquationTransformation::Function(value.0.clone()))
                } else {
                    Some(EquationTransformation::InverseFunction(name.clone()))
                }
            }
            _ => None,
        },
        NodeOrExpression::Expression(expression) => match expression.products.len() {
            0 => None,
            1 => match one_product(expression.products.first().unwrap(), &mut constraints) {
                Some((multiply, side_with_variable)) => Some(EquationTransformation::Multiply {
                    multiply,
                    side_with_variable,
                }),
                None => None,
            },
            _ => match multiple_products(expression) {
                Some((add, side_with_variable)) => Some(EquationTransformation::Add {
                    add,
                    side_with_variable,
                }),
                None => None,
            },
        },
    };

    match inverse {
        Some(transformation) => Some(TransformResult {
            transformation,
            constraints,
        }),
        None => None,
    }
}

fn one_product(product: &Product, constraints: &mut Vec<String>) -> Option<(Product, Product)> {
    let mut new_product = Product::new(vec![], vec![]);
    let mut pr_with_variable = Product::new(vec![], vec![]);

    for (side_pos, side) in [&product.numerator, &product.denominator]
        .into_iter()
        .enumerate()
    {
        for pr_elem in side {
            match &pr_elem.cache {
                Some(cache) => match cache.variables.len() {
                    0 => {
                        if side_pos == 0 {
                            new_product.denominator.push(pr_elem.clone());
                        } else if side_pos == 1 {
                            new_product.numerator.push(pr_elem.clone());
                        } else {
                            panic!("Side position is wrong");
                        }
                    }
                    1 => {
                        if side_pos == 0 {
                            pr_with_variable.numerator.push(pr_elem.clone());
                        } else if side_pos == 1 {
                            pr_with_variable.denominator.push(pr_elem.clone());
                        } else {
                            panic!("Side position is wrong");
                        }
                    }
                    _ => panic!("Too many variables when getting inverse"),
                },
                None => panic!("Element should be analyzed when applying inverse"),
            }
        }
    }

    if new_product.numerator.is_empty() && new_product.denominator.is_empty() {
        None
    } else {
        // debug!("{new_product:#?}");

        for side in [&new_product.numerator, &new_product.denominator] {
            match side.first() {
                Some(first_elem) => {
                    let mut result = first_elem.to_string();

                    for elem in side.iter().skip(1) {
                        result += &format!(" * {elem}");
                    }

                    constraints.push(format!("{result} != 0"));
                }
                None => (),
            };
        }

        Some((new_product, pr_with_variable))
    }
}

fn multiple_products(expression: &Expression) -> Option<(Expression, Expression)> {
    let mut new_expression = Expression::new(vec![]);
    let mut expr_with_variable = Expression::new(vec![]);

    // debug!("{expression:#?}");
    // debug!("{}", expression.rpn());

    for product in &expression.products {
        let mut skip_product = false;
        // debug!("{}", product.rpn());

        if product.numerator.len() == 0 && product.denominator.len() == 0 {
            break;
        }

        for side in [&product.numerator, &product.denominator] {
            for pr_elem in side {
                match &pr_elem.cache {
                    Some(cache) => {
                        if cache.variables.len() == 1 {
                            skip_product = true;
                            break;
                        }
                    }
                    None => panic!("Element should be analyzed when applying inverse"),
                }
            }
        }

        if skip_product {
            expr_with_variable.products.push(product.clone());
        } else {
            let mut new_product = product.clone();
            let pr_elem = if new_product.numerator.len() > 0 {
                new_product.numerator.first_mut().unwrap()
            } else if new_product.numerator.len() > 0 {
                new_product.denominator.first_mut().unwrap()
            } else {
                panic!("Product shouldn't be empty");
            };

            pr_elem.sign = pr_elem.sign * Sign::Negative;
            new_expression.products.push(new_product);
        }
    }

    // debug!("{new_expression}");

    match new_expression.products.len() {
        0 => None,
        _ => Some((new_expression, expr_with_variable)),
    }
}

fn transform_equation(equation: &mut Equation, inverse: EquationTransformation) {
    // debug!("{inverse:#?}");

    for side in &mut equation.equation_sides {
        let is_side_with_variable = if let Some(cache) = &side.cache {
            match cache.variables.len() {
                0 => false,
                1 => true,
                _ => panic!("Too many variables"),
            }
        } else {
            panic!("Not analyzed");
        };

        match &inverse {
            EquationTransformation::Function(name) => {
                if is_side_with_variable {
                    if let NodeOrExpression::Node(Node::Function { name: _, arguments }) =
                        &side.node_or_expression
                    {
                        *side = arguments.get(0).expect("Function with more than one argument can't be solved at the moment").clone();
                    } else {
                        panic!("Wrong transformation")
                    }
                } else {
                    let func = Node::Function {
                        name: name.clone(),
                        arguments: vec![side.clone()],
                    };

                    *side = Element::new(Sign::Positive, NodeOrExpression::Node(func));
                }
            }
            EquationTransformation::InverseFunction(name) => {
                if is_side_with_variable {
                    if let NodeOrExpression::Node(Node::Function { name: _, arguments }) =
                        &side.node_or_expression
                    {
                        *side = arguments.get(0).expect("Function with more than one argument can't be solved at the moment").clone();
                    } else {
                        panic!("Wrong transformation")
                    }
                } else {
                    let negative_one = Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Number(num::BigRational::from_integer(
                            (-1).into(),
                        ))),
                    );

                    let func = Element::new(
                        Sign::Positive,
                        NodeOrExpression::Node(Node::Function {
                            name: name.clone(),
                            arguments: vec![side.clone()],
                        }),
                    );

                    let inverse_func = Node::Power {
                        base: Box::new(func),
                        power: Box::new(negative_one),
                    };

                    *side = Element::new(Sign::Positive, NodeOrExpression::Node(inverse_func));
                }
            }
            EquationTransformation::Multiply {
                multiply,
                side_with_variable,
            } => {
                let product = if is_side_with_variable {
                    side_with_variable.clone()
                } else {
                    let mut new_product = multiply.clone();
                    new_product.numerator.push(side.clone());
                    new_product
                };

                *side = Element::new(
                    // TODO: sign
                    Sign::Positive,
                    NodeOrExpression::Expression(Expression::new(vec![product])),
                );
            }
            EquationTransformation::Add {
                add,
                side_with_variable,
            } => {
                let expression = if is_side_with_variable {
                    side_with_variable.clone()
                } else {
                    let mut new_expr = add.clone();
                    new_expr
                        .products
                        .push(Product::new(vec![side.clone()], vec![]));
                    new_expr
                };

                *side = Element::new(Sign::Positive, NodeOrExpression::Expression(expression));
            }
        }
    }

    // debug!("{equation:#?}");
    // debug!("{}", equation.rpn());
}
