use petgraph::{
    graph::UnGraph,
    stable_graph::{EdgeIndex, NodeIndex},
};

use crate::ast::Equation;

#[derive(Debug, Clone)]
pub struct EquationGraph {
    pub graph: UnGraph<Equation, Vec<String>>,
}

impl EquationGraph {
    pub fn new(equation: Equation) -> (EquationGraph, NodeIndex) {
        let mut graph = EquationGraph {
            graph: Default::default(),
        };

        let index = graph.graph.add_node(equation);

        (graph, index)
    }

    pub fn add_path(
        &mut self,
        equation: Equation,
        constraints: Vec<String>,
        index: NodeIndex,
    ) -> (NodeIndex, EdgeIndex) {
        let node_index = self.graph.add_node(equation);
        let edge_index = self.graph.add_edge(index.into(), node_index, constraints);

        (node_index, edge_index)
    }
}
