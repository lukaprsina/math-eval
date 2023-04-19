use crate::ast::{product::Product, Element, Equation, Expression, Node, NodeOrExpression, Sign};

use super::strategy::Strategy;

// assume that it has been analysed
fn simplify_equation(equation: &mut Equation) -> Vec<String> {
    for side_element in &mut equation.equation_sides {
        // debug!("{}", side_element.rpn());

        side_element.apply_to_every_element_mut(
            &mut |element| {
                let node_or_expression = match &mut element.node_or_expression {
                    NodeOrExpression::Expression(expression) => {
                        let mut new_expr = Expression::new(vec![]);
                        for product in &mut expression.products {
                            let rationalized = product.rationalize();
                            new_expr.products.push(rationalized);
                        }

                        NodeOrExpression::Expression(new_expr)
                    }
                    _ => element.node_or_expression.clone(),
                };

                *element = Element::new(element.sign, node_or_expression);
            },
            false,
            None,
        );

        // debug!("{}", side_element.rpn());

        side_element.analyze(None);

        side_element.apply_to_every_element_mut(
            &mut |element| {
                // debug!("{element} {}", element.is_number());
                // debug!("{}", element.rpn());
                if let NodeOrExpression::Expression(expression) = &mut element.node_or_expression {
                    for product in &mut expression.products {
                        let mut delete_denominator = false;

                        if product.denominator.len() == 1 {
                            let pr_elem = product.denominator.first_mut().unwrap();

                            if let NodeOrExpression::Node(Node::Number(number)) =
                                &mut pr_elem.node_or_expression
                            {
                                if *number == num::BigRational::from_integer(1.into()) {
                                    // debug!("{}", pr_elem.rpn());
                                    delete_denominator = true;
                                }
                            }
                        }

                        if delete_denominator {
                            product.denominator.clear();
                            // debug!("{} {delete_denominator}", product.rpn());
                        }
                    }
                }
            },
            false,
            None,
        );

        let cloned_elem = side_element.clone();

        // debug!("{}", cloned_elem.rpn());

        *side_element = cloned_elem.apply_to_every_element_into(
            &mut |element| {
                // debug!("{}", element.rpn());
                let node_or_expression = match element.node_or_expression {
                    NodeOrExpression::Node(node) => NodeOrExpression::Node(node),
                    NodeOrExpression::Expression(expression) => {
                        let mut new_expression = Expression::new(vec![]);

                        for product in expression.products {
                            let mut pr_stage1 = Product::new(vec![], vec![]);
                            let mut keep_product = true;

                            if product.numerator.len() == 1 && product.denominator.len() == 0 {
                                if let NodeOrExpression::Node(Node::Number(number)) =
                                    &product.numerator.first().unwrap().node_or_expression
                                {
                                    if *number == num::BigRational::from_integer(0.into()) {
                                        // remove zero from x + 0
                                        keep_product = false;
                                    }
                                }
                            }

                            if keep_product {
                                pr_stage1 = product.clone();
                            }

                            let mut pr_stage2 = Product::new(vec![], vec![]);

                            for (side_pos, side) in [pr_stage1.numerator, pr_stage1.denominator]
                                .into_iter()
                                .enumerate()
                            {
                                if side.len() == 1 {
                                    let first_elem = side[0].clone();

                                    if side_pos == 0 {
                                        pr_stage2.numerator.push(first_elem);
                                    } else if side_pos == 1 {
                                        pr_stage2.denominator.push(first_elem);
                                    } else {
                                        panic!("Too many sides");
                                    }
                                    break;
                                }

                                for pr_elem in side {
                                    // debug!("{pr_elem:#?}");
                                    let mut keep_elem = true;

                                    if let NodeOrExpression::Node(Node::Number(number)) =
                                        &pr_elem.node_or_expression
                                    {
                                        if *number == num::BigRational::from_integer(1.into()) {
                                            // remove zero from x*1
                                            keep_elem = false;
                                        }
                                    }

                                    if keep_elem {
                                        // debug!("{pr_elem:#?}");
                                        if side_pos == 0 {
                                            pr_stage2.numerator.push(pr_elem);
                                        } else if side_pos == 1 {
                                            pr_stage2.denominator.push(pr_elem);
                                        } else {
                                            panic!("Too many sides");
                                        }
                                    }
                                }
                            }

                            // debug!("{pr_stage2:#?}");

                            if keep_product {
                                new_expression.products.push(pr_stage2);
                            }
                        }

                        NodeOrExpression::Expression(new_expression)
                    }
                };

                Element::new(element.sign, node_or_expression)
            },
            false,
            None,
        );

        // debug!("{}", side_element.rpn());
        // debug!("{side_element:#?}");

        if let NodeOrExpression::Expression(expression) = &side_element.node_or_expression {
            if expression.products.len() == 0 {
                *side_element = Element::new(
                    Sign::Positive,
                    NodeOrExpression::Node(Node::Number(num::BigRational::from_integer(0.into()))),
                )
            }
        }
    }

    vec![]
}

pub fn get_simplify() -> Strategy {
    Strategy {
        apply: Some(Box::new(simplify_equation)),
        check: None,
    }
}
