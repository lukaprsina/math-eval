use tracing::debug;

use crate::ast::{Element, Equation, Node, NodeOrExpression};

use super::strategy::Strategy;

// imply that it has been analysed
fn solve_one_variable(equation: &mut Equation, variable_name: &str) {
    if equation.equation_sides.len() != 2 {
        return;
    }

    for side_element in &mut equation.equation_sides {
        match &mut side_element.cache {
            Some(cache) => {
                if cache.variables.len() > 1 {
                    break;
                }

                // get to variable
                // keep track of the operations
                // then do the inverse
                build_stack(side_element, variable_name);
                println!("\n");
            }
            None => panic!("Equation has not been analyzed, cannot simplify"),
        }
    }
}

pub fn get_solve_one_variable() -> Strategy {
    Strategy {
        equation: Some(Box::new(solve_one_variable)),
    }
}

fn build_stack(side: &Element, variable_name: &str) -> Vec<Element> {
    let mut stack = vec![];

    side.apply_to_every_element(
        &mut |element| {
            // a
            if let NodeOrExpression::Node(Node::Variable(name)) = &element.node_or_expression {
                if variable_name == name {
                    for element in &stack {
                        debug!("{element}");
                    }
                } else {
                    stack.push(element.clone());
                }
            } else {
                stack.push(element.clone());
            }
        },
        true,
        None,
    );

    stack
}
/*let mut stack = vec![];

    let mut element = side;

    loop {
        match &element.node_or_expression {
            NodeOrExpression::Node(node) => {
                match node {
                    Node::Variable(name) => {
                        if name == variable_name {
                            break;
                        }
                    }
                    _ => (),
                }
                // stack.push(node.clone());
            }
            NodeOrExpression::Expression(expression) => {
                for product in &expression.products {
                    for product_elem in &product.numerator {
                        if let Some(cache) = &product_elem.cache {
                            if cache.variables.contains(variable_name) {
                                stack.push(element.clone());
                                element = &product_elem;
                            }
                            // here
                        }
                    }
                }
            }
        }
        break;
    }

    stack
} */
