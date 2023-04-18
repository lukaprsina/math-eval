use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::tokenizer::parser::ParseError;

use super::{app::App, token_to_element::TokensToEquationError, Equation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Domain {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionProperty {
    Idempotent,
    Involution,
    Asociative,
    Commutative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementDefinition {
    Variable {
        domain: Domain,
    },
    Function {
        domain: Domain,
        codomain: Domain,
        properties: Vec<FunctionProperty>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing, skip_deserializing)]
    pub app: Rc<RefCell<App>>,
    pub equations: HashMap<Uuid, Equation>,
    pub definitions: HashMap<String, ElementDefinition>,
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
            equations: HashMap::new(),
            app,
            uuid: Uuid::nil(),
            definitions: HashMap::new(),
        }
    }

    pub fn get_equation(&self, uuid: Uuid) -> Option<&Equation> {
        self.equations.get(&uuid)
    }

    pub fn get_equation_mut(&mut self, uuid: Uuid) -> Option<&mut Equation> {
        let mut equation = self.equations.get_mut(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub fn remove_equation(&mut self, uuid: Uuid) -> Option<Equation> {
        let mut equation = self.equations.remove(&uuid);

        if let Some(eq) = &mut equation {
            eq.cache = None;
        }

        equation
    }

    pub(crate) fn insert_equation(&mut self, equation: Equation) -> Uuid {
        let uuid = Uuid::new_v4();
        self.equations.insert(uuid, equation);
        uuid
    }

    pub fn analyze(&mut self) -> ContextAnalysis {
        let mut analysis = ContextAnalysis::new();

        for (_, equation) in &mut self.equations {
            for element in &mut equation.equation_sides {
                element.analyze(Some(&mut analysis));
            }
        }

        analysis
    }
}

#[derive(Debug)]
pub enum VariableType {
    Constant,
    Independet,
    Dependent,
}

#[derive(Debug)]
pub enum FunctionType {
    BuiltIn,
    Custom,
}

#[derive(Default, Debug)]
pub struct ContextAnalysis {
    pub variables: HashMap<String, Option<VariableType>>,
    pub functions: HashMap<String, Option<FunctionType>>,
}

impl ContextAnalysis {
    pub fn new() -> ContextAnalysis {
        ContextAnalysis {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}
