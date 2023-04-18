use serde::{Deserialize, Serialize};

use super::{
    element::{IsTimesVisible, ShouldBeParenthesized},
    product::Product,
    Element,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Expression {
    pub products: Vec<Product>,
}

impl Expression {
    pub fn new(products: Vec<Product>) -> Self {
        Expression { products }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl IsTimesVisible for Expression {
    fn is_times_visible(&self, last: &Element) -> bool {
        if !self.products.is_empty() {
            self.products[0].is_times_visible(last)
        } else {
            true
        }
    }
}

impl ShouldBeParenthesized for Expression {
    fn should_be_parenthesized(&self) -> bool {
        if self.products.len() == 1 {
            self.products[0].should_be_parenthesized()
        } else {
            true
        }
    }
}
