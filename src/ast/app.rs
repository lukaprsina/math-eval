use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use petgraph::stable_graph::NodeIndex;
use tracing::debug;
use uuid::Uuid;

use crate::{
    actions::{
        is_same::{IsSame, IsSameNames},
        strategies::strategy::Strategy,
    },
    graph::graph::EquationGraph,
    output::equation_to_rpn::ReversePolishNotation,
};

use super::{
    context::{Context, CreateEquationError},
    equation::NoContextEquation,
    Element, Equation,
};

const STRATEGIES: [&'static str; 1] = ["apply_inverse"];

#[derive(Debug)]
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

    pub fn solve(&mut self, context_uuid: Uuid) {
        // println!("Context {}", self.uuid);
        let mut context = self
            .remove_context(context_uuid)
            .expect("Context not found");

        for (_, equation) in &mut context.equations {
            self.solve_equation(equation);
        }

        self.contexts.insert(context_uuid, context);
        // println!("Analysis: {:#?}", analysis);
    }

    pub fn solve_equation(&mut self, equation: &mut Equation) {
        let (mut graph, center_index) = EquationGraph::new(equation.clone());
        self.process_graph_node(center_index, &mut graph);
    }

    pub fn process_graph_node(
        &mut self,
        node_index: NodeIndex,
        graph: &mut EquationGraph,
    ) -> Vec<NodeIndex> {
        let mut original_eq = graph.graph[node_index].clone();
        // debug!("{}", original_eq.rpn());

        let mut indices = vec![];

        for strategy in STRATEGIES {
            for side in &mut original_eq.equation_sides {
                side.analyze(None);
            }

            let mut previous_eq = original_eq.clone();
            loop {
                for strategy in ["flatten", "simplify"] {
                    original_eq.apply_strategy(self, strategy);
                    // debug!("{}", original_eq.rpn());
                }

                let mut names = IsSameNames::new();
                let is_same = IsSame::is_same(&previous_eq, &original_eq, &mut names);
                if is_same && names.check() {
                    debug!("{}", original_eq.rpn());
                    break;
                }

                previous_eq = original_eq.clone();
            }

            for side in &mut original_eq.equation_sides {
                side.analyze(None);
            }

            let mut cloned_eq = original_eq.clone();

            let constraints = cloned_eq.apply_strategy(self, strategy);
            // debug!("{}", cloned_eq.rpn());

            let (node_index, _) = graph.add_path(cloned_eq.clone(), constraints, node_index);

            let leaf_eq = &graph.graph[node_index];
            let mut names = IsSameNames::new();
            let is_same = IsSame::is_same(leaf_eq, &original_eq, &mut names);
            if !is_same || !names.check() {
                indices.push(node_index);
            } else {
                debug!("{}", original_eq.rpn());
            }
        }

        let mut new_indices = vec![];
        for index in indices {
            let leaves = self.process_graph_node(index, graph);
            new_indices.extend(leaves);
        }

        graph.graph[node_index] = original_eq;
        new_indices
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
