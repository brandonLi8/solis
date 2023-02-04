// Copyright © 2022 Brandon Li. All rights reserved.

//! Generally speaking, register allocation is a compiler optimization that attempts to utilize as many registers as
//! possible for sub-expressions and variables before resorting to utilizing the stack.
//!
//! Part of the problem has been solved by translating the input program into the Intermediate Representation, which
//! flattens sub-expressions into temporary variables (see `ir.rs`). This is more conducive to the register allocation,
//! which is now reduced to allocating for variables only.
//!
//! The allocator works by analyzing each block of the IR and creating an assignment of each variable into either a
//! register or the stack (the location within the stack is handled in the symbol table at the compiler step). We use
//! Chaitin's algorithm to create a correct and reasonable assignment. On a high level, the algorithm works by creating
//! a graph where each variable is a node and each edge represents a conflict between the variables. A conflict
//! is when both variable's lifetimes intersect at some point, meaning they cannot be assigned the same register. Then,
//! we k-color the graph to achieve an assignment where neighboring edges have different assignments.

use asm::asm::Register;
use error_messages::internal_compiler_error;
use ir::ir::Block;
use register_allocation::conflict_analysis::conflict_analysis;

/// An assignment of where to evaluate an expression.
#[derive(Debug)]
pub enum Assignment {
    Register(Register),
    Spill,
}

/// Creates an assignment of registers for each variable in the block.
/// * block - the block to create the allocation for
/// * registers - the pool of registers to assign variables to
/// Returns a map of variable names to the corresponding assignment.
pub fn allocate_registers<'a>(block: &'a Block, registers: Set<&'a Register>) -> Map<&'a String, Assignment> {
    // Create a interference graph and frequency map
    let (mut interference_graph, mut variable_frequencies) = conflict_analysis(block);

    // A stack that contains variables that are color-able. Please see
    // https://stackoverflow.com/questions/14399608/chaitin-briggs-algorithm-explanation for an overview of the
    // algorithm.
    let mut colorable_nodes_stack: Vec<&String> = vec![];

    // Result allocation.
    let mut allocation = Map::new();

    while interference_graph.size() > 0 {
        // Find a node N with degree less than R = registers.length
        if let Some((node, _)) = interference_graph
            .nodes
            .iter()
            .find(|(_, neighbors)| neighbors.len() < registers.len())
        {
            // Remove N and its associated edges from G and push N on a stack S
            variable_frequencies.remove(node);
            colorable_nodes_stack.push(node);
            interference_graph.remove_node(node);
        } else {
            // Otherwise the graph cannot be colored with R colors. Simplify the graph G by choosing a variable to
            // spill and remove its node N from G.
            // We spill the node that has the minimum heuristic.
            let (&spilled_node, _) = variable_frequencies
                .iter()
                .min_by_key(|(_, frequency)| spill_heuristic(**frequency))
                .unwrap_or_else(|| internal_compiler_error("no variable to spill"));

            // Remove the spilled node.
            interference_graph.remove_node(spilled_node);
            variable_frequencies.remove(&spilled_node);

            allocation.insert(spilled_node, Assignment::Spill);
        }
    }

    // While stack S contains a node N, Add N to graph G and assign it a color from the R colors
    while let Some(node) = colorable_nodes_stack.pop() {
        // Set of the registers that it's neighbors were assigned to.
        let mut neighbor_registers = Set::new();

        for neighbor in interference_graph.neighbors_when_removed(node) {
            if let Some(Assignment::Register(register)) = allocation.get(neighbor) {
                neighbor_registers.insert(register);
            }
        }

        // Assign the node any register in the available pool (all registers - neighbor registers)
        let register = **registers
            .difference(&neighbor_registers)
            .next()
            .unwrap_or_else(|| internal_compiler_error("no available register for color-able node"));
        allocation.insert(node, Assignment::Register(register));
    }

    allocation
}

// Defines the Spill metric of a variable. See https://en.wikipedia.org/wiki/Spill_metric. Smaller means more likely to be spilled.
// * frequency - the frequency (# of times it is referenced after definition) of the variable
const fn spill_heuristic(frequency: usize) -> usize {
    // Currently, our spill metric spills the variable that is referenced the least. In the future, consider
    // adding a weighted sum of the frequency (normalized) and the inverse degree of the variable.
    frequency
}

// For the compiler, we want to use Hash tables for performance. However for testing, we need the result of the
// register allocation (steps) to be deterministic. Create an aliased type that is stubbed based on the test environment
#[cfg(not(feature = "test"))]
pub type Set<T> = std::collections::HashSet<T>;

#[cfg(feature = "test")]
pub type Set<T> = std::collections::BTreeSet<T>;

#[cfg(not(feature = "test"))]
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[cfg(feature = "test")]
pub type Map<K, V> = std::collections::BTreeMap<K, V>;
