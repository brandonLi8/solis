// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Performs variable conflict analysis. Conflict analysis calculates a graph (called the `InterferenceGraph`) where
//! each variable is a node and edges represents conflicts, which is when two variables cannot be assigned the same
//! register.
//!
//! In other words, a conflict (an edge) is when both variable's lifetimes intersect at some point, meaning they cannot
//! be assigned the same register. This is the approach that is used to create the interference graph: by performing
//! liveness analysis (see `liveness_analysis.rs`) on each expression in a block (in reverse order), and the
//! variables that are returned in the liveness set are added to the graph as a strongly connected component.

use error_messages::internal_compiler_error;
use ir::ir::{Block, Type};
use register_allocation::liveness_analysis::liveness_analysis;
use register_allocation::register_allocator::{Map, Set};

/// Struct representing the interference (conflict) graph. Nodes can only be added and removed once.
#[derive(Debug)]
pub struct InterferenceGraph<'a> {
    // The nodes of the graph, represented as an adjacency matrix (variable to set of neighbors)
    pub nodes: Map<&'a String, Set<&'a String>>,

    // Nodes that have been removed, mapped to the neighbors that each node had when it was removed. This is needed
    // for Chaitin's algorithm (see `register_allocator.rs`).
    removed_nodes: Map<&'a String, Set<&'a String>>,
}

impl<'a> InterferenceGraph<'a> {
    /// `InterferenceGraph` constructor.
    pub fn new() -> Self {
        InterferenceGraph { nodes: Map::new(), removed_nodes: Map::new() }
    }

    /// Adds an node to the graph.
    pub fn add_node(&mut self, variable: &'a std::string::String) {
        if self.nodes.get(&variable).is_none() {
            self.nodes.insert(variable, Set::new());
        }
    }

    /// Adds an edge from `variable_1` to `variable_2`.
    pub fn add_edge(&mut self, variable_1: &'a std::string::String, variable_2: &'a std::string::String) {
        self.add_node(variable_1);
        self.add_node(variable_2);

        self.nodes.get_mut(&variable_1).unwrap().insert(variable_2);
        self.nodes.get_mut(&variable_2).unwrap().insert(variable_1);
    }

    /// Removes a node from the graph. Once a node has been removed, it cannot be re-added.
    pub fn remove_node(&mut self, variable: &'a String) {
        self.removed_nodes.insert(
            variable,
            self.nodes
                .remove(variable)
                .unwrap_or_else(|| internal_compiler_error(&format!("node {variable} not found"))),
        );

        for node in self.nodes.values_mut() {
            node.remove(variable);
        }
    }

    /// Gets the size of the graph.
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    /// Removes a node from the graph. Once a node has been removed, it cannot be re-added.
    pub fn neighbors_when_removed(&self, variable: &'a String) -> &Set<&String> {
        self.removed_nodes
            .get(variable)
            .unwrap_or_else(|| internal_compiler_error("variable was never removed"))
    }
}

/// Performs conflict analysis for the passed in block.
/// * return: (
/// *  - the `InterferenceGraph`
/// *  - the `InterferenceGraph` for variables of type `Type::Float`
/// *  - the variable frequencies map
/// * )
pub fn conflict_analysis<'a>(
    block: &'a Block,
    params: &Set<&'a String>,
) -> (InterferenceGraph<'a>, InterferenceGraph<'a>, Map<&'a String, usize>) {
    let mut live_variables = Map::new();
    let mut variable_frequencies = Map::new();
    let mut interference_graph = InterferenceGraph::new();

    // Create a separate interference graph for floating point variables, since floats use a different register set.
    let mut float_interference_graph = InterferenceGraph::new();

    conflict_analysis_block(
        block,
        params,
        &mut live_variables,
        &mut variable_frequencies,
        &mut interference_graph,
        &mut float_interference_graph,
    );

    (interference_graph, float_interference_graph, variable_frequencies)
}

/// Performs conflict analysis, given a interference graph to add onto
/// * `live_variables` - starting live variables
/// * `variable_frequencies` - starting `variable_frequencies`
/// * `interference_graph` - interference graph to add onto
pub fn conflict_analysis_block<'a>(
    block: &'a Block,
    params: &Set<&'a String>,
    live_variables: &mut Map<&'a String, &'a Type>,
    variable_frequencies: &mut Map<&'a String, usize>,
    interference_graph: &mut InterferenceGraph<'a>,
    float_interference_graph: &mut InterferenceGraph<'a>,
) {
    for expr in block.exprs.iter().rev() {
        liveness_analysis(
            expr,
            live_variables,
            variable_frequencies,
            params,
            interference_graph,
            float_interference_graph,
        );

        // For each pair of variables that are live (at this point), add a conflict between them
        for (i, (variable_1, variable_1_type)) in live_variables.iter().enumerate() {
            let variable_1_is_float = matches!(variable_1_type, Type::Float);

            // Add the node to ensure that it is in the conflict graph (even if it doesn't conflict with something).
            if variable_1_is_float {
                float_interference_graph.add_node(variable_1);
            } else {
                interference_graph.add_node(variable_1);
            }

            for (j, (variable_2, variable_2_type)) in live_variables.iter().enumerate() {
                if i < j {
                    let variable_2_is_float = matches!(variable_2_type, Type::Float);

                    if !variable_1_is_float && !variable_2_is_float {
                        interference_graph.add_edge(variable_1, variable_2);
                    } else if variable_1_is_float && variable_2_is_float {
                        float_interference_graph.add_edge(variable_1, variable_2);
                    }
                }
            }
        }
    }
}
