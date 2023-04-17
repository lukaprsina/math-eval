use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::tokenizer::{parser::TokenizedString, token::Operation};

use super::{app::App, context::CreateEquationError, Element};

#[derive(Debug, Clone)]
pub struct EquationCache {}

#[derive(Debug, Clone)]
pub struct Equation {
    pub equation_sides: Vec<Element>,
    pub app: Rc<RefCell<App>>,
    pub context: Uuid,
    pub cache: Option<EquationCache>,
}

pub struct NoContextEquation {
    pub sides: Vec<EquationSide>,
}

#[derive(Debug, Clone)]
pub struct EquationSide {
    pub element: Element,
    pub operation: Option<Operation>,
}

impl Equation {
    pub fn new(elements: Vec<Element>, app: Rc<RefCell<App>>, ctx_uuid: Uuid) -> Self {
        let equation = Equation {
            equation_sides: elements,
            app: Rc::clone(&app),
            context: ctx_uuid,
            cache: Some(EquationCache {}),
        };

        // info!("{}", equation);
        // equation.flatten()
        equation
        // println!("{:#?}", equation);
    }

    pub fn apply_strategy(&mut self, app: &mut App, strategy_name: &str) -> Vec<String> {
        let mut strategy = app.strategies.remove(strategy_name).unwrap();

        let func = &mut strategy.apply.as_deref_mut().unwrap();
        let constraints = func(self);

        app.strategies.insert(strategy_name.to_string(), strategy);
        constraints
    }
}

impl EquationSide {
    pub fn new(element: Element, operation: Option<Operation>) -> Self {
        Self { element, operation }
    }
}

impl NoContextEquation {
    pub fn new(sides: Vec<EquationSide>) -> Self {
        NoContextEquation { sides }
    }
}

impl TryFrom<&str> for NoContextEquation {
    type Error = CreateEquationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tokens = TokenizedString::try_from(value).map_err(CreateEquationError::ParseError)?;
        // println!("{}\n{:#?}\n", value, tokens.tokens);

        let ast = NoContextEquation::try_from(tokens)
            .map_err(CreateEquationError::TokensToEquationError)?;

        Ok(ast)
    }
}
