pub mod context;
pub mod debug_print;
pub mod equation;
pub mod equation_to_string;
pub mod expression;
pub mod node;
pub mod token_to_node;
// pub mod flatten;

pub use {
    equation::Equation,
    expression::{Expression, NodeOrExpression, Product, Sign},
    node::Node,
};
