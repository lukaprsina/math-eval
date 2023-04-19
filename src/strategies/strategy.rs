use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::ast::{app::App, Equation};

use super::{apply_inverse, flatten, simplify};

#[derive(Serialize, Deserialize)]
pub struct Strategy {
    #[serde(skip_serializing, skip_deserializing)]
    pub check: Option<Box<dyn FnMut(&mut Equation) -> bool>>,
    #[serde(skip_serializing, skip_deserializing)]
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
            ("simplify", simplify::get_simplify()),
            ("apply_inverse", apply_inverse::get_apply_inverse()),
            ("flatten", flatten::get_flatten()),
        ];

        self.strategies.extend(
            tuples
                .into_iter()
                .map(|tuple| (tuple.0.to_string(), tuple.1))
                .collect::<HashMap<String, Strategy>>(),
        );
    }
}
