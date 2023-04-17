use std::{collections::HashMap, fmt::Debug};

use crate::{
    actions::strategies,
    ast::{app::App, Equation},
};

pub struct Strategy {
    pub check: Option<Box<dyn FnMut(&mut Equation) -> bool>>,
    pub apply: Option<Box<dyn FnMut(&mut Equation) -> Vec<String>>>,
}

impl Debug for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Strategy").finish()
    }
}

impl App {
    pub fn add_strategies(&mut self) {
        let tuples = [
            ("simplify", strategies::simplify::get_simplify()),
            (
                "apply_inverse",
                strategies::apply_inverse::get_apply_inverse(),
            ),
            ("flatten", strategies::flatten::get_flatten()),
        ];

        self.strategies.extend(
            tuples
                .into_iter()
                .map(|tuple| (tuple.0.to_string(), tuple.1))
                .collect::<HashMap<String, Strategy>>(),
        );
    }
}
