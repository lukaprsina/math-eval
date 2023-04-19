use petgraph::{
    dot::{Config, Dot},
    stable_graph::NodeIndex,
};
use tracing::debug;
use uuid::Uuid;

use crate::{
    actions::is_same::{IsSame, IsSameNames},
    graph::graph::EquationGraph,
    output::equation_to_rpn::ReversePolishNotation,
};

use crate::ast::{app::App, Equation};

const STRATEGIES: [&'static str; 1] = ["apply_inverse"];

impl App {
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

        let dot_format = Dot::with_config(&graph.graph, &[Config::EdgeNoLabel]);
        debug!("{dot_format:?}");
        debug!("{dot_format:#?}");

        println!("\n");
        let graph_json = serde_json::to_string_pretty(&graph.graph).unwrap();
        debug!("{graph_json:?}");
        debug!("{graph_json:#?}");
        debug!("{graph_json}");
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
}
