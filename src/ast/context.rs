use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use thiserror::Error;
use uuid::Uuid;

use crate::{ast::analyzed_expression::AnalyzedElement, tokenizer::parser::ParseError};

use super::{app::App, token_to_element::TokensToEquationError, Equation};

#[derive(Debug, Clone)]
pub struct Context {
    pub app: Rc<RefCell<App>>,
    pub elements: HashMap<Uuid, AnalyzedElement>,
    pub equations: HashMap<Uuid, Equation>,
    pub uuid: Uuid,
}

#[derive(Debug, Error)]
pub enum CreateEquationError {
    #[error("{0}")]
    ParseError(ParseError),
    #[error("{0}")]
    TokensToEquationError(TokensToEquationError),
}

impl Context {
    pub fn new(app: Rc<RefCell<App>>) -> Context {
        Context {
            elements: HashMap::new(),
            equations: HashMap::new(),
            app,
            uuid: Uuid::nil(),
        }
    }

    pub fn get_element(&self, uuid: Uuid) -> Option<&AnalyzedElement> {
        self.elements.get(&uuid)
    }

    pub fn get_element_mut(&mut self, uuid: Uuid) -> Option<&mut AnalyzedElement> {
        self.elements.get_mut(&uuid)
    }

    pub fn solve(&mut self) {
        /* println!("Context:");
        for (uuid, analyzed_element) in self.elements.iter() {
            println!(
                "{}: {}\nIs number?: {}\n{:#?}\n\n",
                uuid, analyzed_element.element, analyzed_element.is_number, analyzed_element.info
            );
        } */
    }
}
