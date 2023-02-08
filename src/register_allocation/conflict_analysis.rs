// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Performs variable conflict analysis. Conflict analysis calculates a graph (called the `InterferenceGraph`) where
//! each variable is a node and edges represents conflicts, which is when two variables cannot be assigned the same
//! register.
//!
//! In other words, a conflict (an edge) is when both variable's lifetimes intersect at some point, meaning they cannot
//! be assigned the same register. This is the approach that is used to create the interference graph: by performing
//! liveness analysis (see `liveness_analysis.rs`) on each expression in a block (in reverse order), and for the
//! variables that are returned in the liveness set are added to the graph as a strongly connected component.

use error_messages::internal_compiler_error;
use ir::ir::Block;
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
    /// Tokens Cursor constructor.
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
        self.removed_nodes
            .insert(variable, self.nodes.remove(variable).unwrap());
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

/// Performs conflict analysis for the passed in block. Returns the `InterferenceGraph` and the variable frequencies map.
pub fn conflict_analysis(block: &Block) -> (InterferenceGraph, Map<&String, usize>) {
    let mut live_variables = Set::<&String>::new();
    let mut variable_frequencies = Map::<&String, usize>::new();
    let mut interference_graph = InterferenceGraph::new();

    for expr in block.exprs.iter().rev() {
        liveness_analysis(expr, &mut live_variables, &mut variable_frequencies);

        for variable_1 in &live_variables {
            for variable_2 in &live_variables {
                if variable_1 != variable_2 {
                    interference_graph.add_edge(variable_1, variable_2);
                }
            }
        }
    }

    // Add each node in variable_frequencies, which ensures that every variable in the block is added to the
    // interference graph (specifically the case of variables that are not referenced after initialization).
    for variable in variable_frequencies.keys() {
        interference_graph.add_node(variable);
    }

    (interference_graph, variable_frequencies)
}
