use itertools::Itertools;

use crate::ast::{product::Product, Element, Equation, Expression, NodeOrExpression, Sign};

use super::strategy::Strategy;

pub fn get_flatten() -> Strategy {
    Strategy {
        apply: Some(Box::new(flatten_equation)),
        check: None,
    }
}

fn flatten_equation(equation: &mut Equation) -> Vec<String> {
    for side_element in &mut equation.equation_sides {
        side_element.apply_to_every_element_mut(&mut flatten_element, false, None);
    }

    vec![]
}

fn flatten_element(element: &mut Element) {
    let sign = element.sign.clone();

    let node_or_expression = match &mut element.node_or_expression {
        NodeOrExpression::Node(node) => NodeOrExpression::Node(node.clone()),
        NodeOrExpression::Expression(expression) => {
            let mut new_expression = Expression::new(vec![]);

            for product in &mut expression.products {
                let is_surrounded = (product.numerator.len() + product.denominator.len()) >= 2;

                let mut new_products: Vec<Product> = vec![];

                for (side_pos, side) in [&mut product.numerator, &mut product.denominator]
                    .into_iter()
                    .enumerate()
                {
                    for inner_element in side {
                        process_inner_element(
                            inner_element,
                            &mut new_products,
                            side_pos,
                            is_surrounded,
                        );
                    }
                }

                new_expression.products.extend(new_products);
            }

            NodeOrExpression::Expression(new_expression)
        }
    };

    *element = Element::new(sign, node_or_expression);
}

fn process_inner_element(
    inner_element: &mut Element,
    new_products: &mut Vec<Product>,
    side_pos: usize,
    is_surrounded: bool,
) {
    // debug!("inner_element: {inner_element:#?}");
    if inner_element.sign != Sign::Positive {
        move_element_to_products(inner_element.clone(), new_products, side_pos);
        return;
    }

    match &mut inner_element.node_or_expression {
        NodeOrExpression::Expression(inner_expression) => {
            if inner_expression.products.len() == 1 {
                transfer_products(inner_expression.clone(), new_products, side_pos);
            } else {
                if is_surrounded {
                    move_element_to_products(inner_element.clone(), new_products, side_pos);
                } else {
                    transfer_elements_in_products(inner_expression.clone(), new_products, side_pos);
                }
            }
        }
        NodeOrExpression::Node(_) => {
            move_element_to_products(inner_element.clone(), new_products, side_pos)
        }
    }
}

fn move_element_to_products(element: Element, new_products: &mut Vec<Product>, side_pos: usize) {
    let mut new_product = if new_products.len() == 0 {
        Product::new(vec![], vec![])
    } else if new_products.len() == 1 {
        new_products.remove(0)
    } else {
        panic!("Product should not have more than one element");
    };

    match side_pos {
        0 => new_product.numerator.push(element),
        1 => new_product.denominator.push(element),
        _ => unreachable!(),
    }

    new_products.push(new_product);
}

fn transfer_products(
    inner_expression: Expression,
    new_products: &mut Vec<Product>,
    side_pos: usize,
) {
    let mut new_product = if new_products.len() == 0 {
        Product::new(vec![], vec![])
    } else if new_products.len() == 1 {
        new_products.remove(0)
    } else {
        panic!("Product should not have more than one element");
    };

    for inner_product in inner_expression.products {
        if side_pos == 0 {
            new_product.numerator.extend(inner_product.numerator);

            new_product.denominator.extend(inner_product.denominator);
        } else if side_pos == 1 {
            new_product.numerator.extend(inner_product.denominator);

            new_product.denominator.extend(inner_product.numerator);
        } else {
            panic!("Too many ratio sides");
        }
    }

    new_products.push(new_product);
}

fn transfer_elements_in_products(
    inner_expression: Expression,
    new_products: &mut Vec<Product>,
    side_pos: usize,
) {
    for inner_product in inner_expression.products {
        for side in [inner_product.numerator, inner_product.denominator] {
            let products = if side_pos == 0 {
                side.into_iter()
                    .map(|elem| Product::new(vec![elem], vec![]))
                    .collect_vec()
            } else if side_pos == 1 {
                side.into_iter()
                    .map(|elem| Product::new(vec![], vec![elem]))
                    .collect_vec()
            } else {
                panic!("Too many ratio sides");
            };

            new_products.extend(products);
        }
    }
}
