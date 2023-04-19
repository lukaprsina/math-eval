use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::strategies::strategy::Strategy;

use super::{
    context::{Context, CreateEquationError},
    equation::NoContextEquation,
    Element, Equation,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    pub formulas: Uuid,
    pub contexts: HashMap<Uuid, Context>,
    pub strategies: HashMap<String, Strategy>,
}

impl App {
    pub fn new() -> Result<Rc<RefCell<App>>, CreateEquationError> {
        let mut app = App {
            formulas: Uuid::nil(),
            contexts: HashMap::new(),
            strategies: HashMap::new(),
        };

        app.add_strategies();

        let app = Rc::new(RefCell::new(app));

        Ok(app)
    }

    pub fn add_context(&mut self, mut context: Context) -> Uuid {
        let uuid = Uuid::new_v4();
        context.uuid = uuid;
        self.contexts.insert(uuid, context);
        uuid
    }

    pub fn get_context(&self, uuid: Uuid) -> Option<&Context> {
        self.contexts.get(&uuid)
    }

    pub fn get_context_mut(&mut self, uuid: Uuid) -> Option<&mut Context> {
        self.contexts.get_mut(&uuid)
    }

    pub fn remove_context(&mut self, uuid: Uuid) -> Option<Context> {
        self.contexts.remove(&uuid)
    }

    pub fn try_add_equation<T: Debug + TryInto<NoContextEquation, Error = CreateEquationError>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Result<Uuid, CreateEquationError> {
        let no_ctx_equation: NoContextEquation = input.try_into()?;

        /* no_ctx_equation
        .sides
        .iter()
        .for_each(|side| println!("{:#?}", side.element)); */

        let equation = App::add_equation(Rc::clone(&app), ctx_uuid, no_ctx_equation);

        Ok(equation)
    }

    pub fn add_equation<T: Into<NoContextEquation>>(
        app: Rc<RefCell<App>>,
        ctx_uuid: Uuid,
        input: T,
    ) -> Uuid {
        let no_ctx_eq: NoContextEquation = input.into();

        let mut elements: Vec<Element> = Vec::new();

        for side in no_ctx_eq.sides {
            // info!("{}", side.element);
            // TODO: ignores operation
            elements.push(side.element);
        }

        let equation = Equation::new(elements, Rc::clone(&app), ctx_uuid);

        {
            let mut borrowed_app = app.borrow_mut();
            let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
            ctx.insert_equation(equation)
        }
    }
}
