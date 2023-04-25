use itertools::Itertools;

use crate::ast::{product::Product, Element, Expression, Node, NodeOrExpression};

impl Element {
    pub fn apply_to_every_element(
        &self,
        function: &mut impl FnMut(&Element),
        top_down: bool,
        max_level: Option<i32>,
    ) {
        let (level, max_level) = match max_level {
            Some(level) => (Some(level - 1), level - 1),
            None => (None, 0),
        };

        if top_down {
            function(self);
        }

        if max_level >= 0 {
            match &self.node_or_expression {
                NodeOrExpression::Node(node) => match node {
                    Node::Power { base, power } => {
                        base.apply_to_every_element(function, top_down, level);
                        power.apply_to_every_element(function, top_down, level);
                    }
                    Node::Modulo { lhs, rhs } => {
                        lhs.apply_to_every_element(function, top_down, level);
                        rhs.apply_to_every_element(function, top_down, level);
                    }
                    Node::Factorial { child } => {
                        child.apply_to_every_element(function, top_down, level);
                    }
                    Node::Function { name: _, arguments } => {
                        for argument in arguments.iter() {
                            argument.apply_to_every_element(function, top_down, level);
                        }
                    }
                    _ => (),
                },
                NodeOrExpression::Expression(expression) => {
                    for product in &expression.products {
                        for side in [&product.numerator, &product.denominator] {
                            for element in side {
                                element.apply_to_every_element(function, top_down, level);
                            }
                        }
                    }
                }
            }
        }

        if !top_down {
            function(self);
        }
    }

    pub fn apply_to_every_element_mut(
        &mut self,
        function: &mut impl FnMut(&mut Element),
        top_down: bool,
        max_level: Option<i32>,
    ) {
        let (level, max_level) = match max_level {
            Some(level) => (Some(level - 1), level - 1),
            None => (None, 0),
        };

        if top_down {
            function(self);
        }
        if max_level >= 0 {
            match &mut self.node_or_expression {
                NodeOrExpression::Node(node) => match node {
                    Node::Power { base, power } => {
                        base.apply_to_every_element_mut(function, top_down, level);
                        power.apply_to_every_element_mut(function, top_down, level);
                    }
                    Node::Modulo { lhs, rhs } => {
                        lhs.apply_to_every_element_mut(function, top_down, level);
                        rhs.apply_to_every_element_mut(function, top_down, level);
                    }
                    Node::Factorial { child } => {
                        child.apply_to_every_element_mut(function, top_down, level);
                    }
                    Node::Function { name: _, arguments } => {
                        for argument in arguments.iter_mut() {
                            argument.apply_to_every_element_mut(function, top_down, level);
                        }
                    }
                    _ => (),
                },
                NodeOrExpression::Expression(expression) => {
                    for product in &mut expression.products {
                        for side in [&mut product.numerator, &mut product.denominator] {
                            for element in side {
                                // info!("{:#?}", element);
                                element.apply_to_every_element_mut(function, top_down, level);
                            }
                        }
                    }
                }
            }
        }

        if !top_down {
            function(self);
        }
    }

    pub fn apply_to_every_element_into(
        mut self,
        function: &mut impl FnMut(Element) -> Element,
        top_down: bool,
        max_level: Option<i32>,
    ) -> Element {
        let (level, max_level) = match max_level {
            Some(level) => (Some(level - 1), level - 1),
            None => (None, 0),
        };

        if top_down {
            self = function(self);
        }

        let mut result = if max_level >= 0 {
            match self.node_or_expression {
                NodeOrExpression::Node(node) => {
                    let new_node = match node {
                        Node::Power { base, power } => Node::Power {
                            base: Box::new(
                                base.apply_to_every_element_into(function, top_down, level),
                            ),
                            power: Box::new(
                                power.apply_to_every_element_into(function, top_down, level),
                            ),
                        },
                        Node::Modulo { lhs, rhs } => Node::Modulo {
                            lhs: Box::new(
                                lhs.apply_to_every_element_into(function, top_down, level),
                            ),
                            rhs: Box::new(
                                rhs.apply_to_every_element_into(function, top_down, level),
                            ),
                        },
                        Node::Factorial { child } => Node::Factorial {
                            child: Box::new(
                                child.apply_to_every_element_into(function, top_down, level),
                            ),
                        },
                        Node::Function { name, arguments } => {
                            let new_args = arguments
                                .into_iter()
                                .map(|argument| {
                                    argument.apply_to_every_element_into(function, top_down, level)
                                })
                                .collect_vec();

                            Node::Function {
                                name,
                                arguments: new_args,
                            }
                        }
                        _ => node,
                    };

                    Element::new(self.sign, NodeOrExpression::Node(new_node))
                }
                NodeOrExpression::Expression(expression) => {
                    let mut new_expression = Expression::new(vec![]);

                    for product in expression.products {
                        let mut new_product = Product::new(vec![], vec![]);

                        for element in product.numerator {
                            new_product.numerator.push(
                                element.apply_to_every_element_into(function, top_down, level),
                            );
                        }
                        for element in product.denominator {
                            new_product.denominator.push(
                                element.apply_to_every_element_into(function, top_down, level),
                            );
                        }

                        new_expression.products.push(new_product);
                    }

                    Element::new(self.sign, NodeOrExpression::Expression(new_expression))
                }
            }
        } else {
            self
        };

        if !top_down {
            result = function(result);
        }

        result
    }
}
