// Copyright © 2022-2023 Brandon Li. All rights reserved.

//! Performs variable liveness analysis. Liveness analysis calculates the lifetime of each variable within each
//! expression in a block. A variable is live at some point if it holds a value that may be needed in the future, or
//! equivalently if its value may be read before the next time the variable is written to.
//!
//! To perform liveness analysis, we start from the back of the block. For each expression, if a variable is used within
//! the expression, it is live. When a variable is first assigned, it is no longer live. This file defines a function
//! that outputs the variables that are live right before the expression "runs".
//!
//! For example:
//!   let a: int = 1 + 2
//!   let b: int = 6
//!   let c: int = 8 + a + b + 7
//!   let d: int = a + c
//!     Line 4 output: {a, c}
//!     Line 3 output: {a, b}   (add b, remove c)
//!     Line 2 output: {a}      (remove, b)
//!     Line 1 output: {}       (remove a)

use ir::ir::{DirectExpr, Expr, Type};
use register_allocation::conflict_analysis::{conflict_analysis_block, InterferenceGraph};
use register_allocation::register_allocator::Map;

/// Computes the variables that are live right before the expression runs. In other words, it computes the variables
/// that are live (needed) to execute this expression (and everything after). It does this by modifying
/// `live_variables`.
///
/// `live_variables` - the variables that are live when the next expression runs, mapped to the type of the variable.
/// `variable_frequencies` - maps variables to the number of times they are referenced. Modified in this function.
pub fn liveness_analysis<'a>(
    expr: &'a Expr,
    live_variables: &mut Map<&'a String, &'a Type>,
    variable_frequencies: &mut Map<&'a String, usize>,
    interference_graph: &mut InterferenceGraph<'a>,
    float_interference_graph: &mut InterferenceGraph<'a>,
) {
    match expr {
        Expr::Direct { expr } => liveness_analysis_direct(expr, live_variables, variable_frequencies),
        Expr::Let { id, init_expr } => {
            liveness_analysis(
                init_expr,
                live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );
            live_variables.remove(id);

            // For variables that are created but never referenced after. These variables still need to be considered
            // and added to the conflict graph for an assignment.
            if variable_frequencies.get(id).is_none() {
                variable_frequencies.insert(id, 0);
            };
        }
        Expr::BinaryExpr { operand_1, operand_2, .. } => {
            liveness_analysis_direct(operand_1, live_variables, variable_frequencies);
            liveness_analysis_direct(operand_2, live_variables, variable_frequencies);
        }
        Expr::UnaryExpr { operand, .. } => {
            liveness_analysis_direct(operand, live_variables, variable_frequencies);
        }
        Expr::TypeCoercion { expr, .. } => {
            liveness_analysis_direct(expr, live_variables, variable_frequencies);
        }
        Expr::If { condition, then_block, else_block: Some(else_block) } => {
            // Perform conflict analysis on both branches. This allows variables that are declared in the branch
            // to be in the interference graph with other variables used in that branch. Note that these variables
            // are not in the set after the conflict analysis is over (since it was declared inside the block!).
            let mut then_live_variables = live_variables.clone();
            let mut else_live_variables = live_variables.clone();

            conflict_analysis_block(
                then_block,
                &mut then_live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );

            conflict_analysis_block(
                else_block,
                &mut else_live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );

            // Add the union of then_live_variables and else_live_variables to live_variables. This means that every
            // variable that is **still** (at the start of the block) live in either branch are in conflict with each
            // other.
            live_variables.extend(then_live_variables);
            live_variables.extend(else_live_variables);

            // Finally do liveness on the condition expression.
            liveness_analysis_direct(condition, live_variables, variable_frequencies);
        }
        Expr::If { condition, then_block, else_block: None } => {
            conflict_analysis_block(
                then_block,
                live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );

            liveness_analysis_direct(condition, live_variables, variable_frequencies);
        }
        Expr::Call { .. } => todo!(),
    }
}

// The same as `liveness_analysis` but for a directs.
fn liveness_analysis_direct<'a>(
    direct: &'a DirectExpr,
    live_variables: &mut Map<&'a String, &'a Type>,
    variable_frequencies: &mut Map<&'a String, usize>,
) {
    if let DirectExpr::Id { value, id_type } = direct {
        live_variables.insert(value, id_type);

        *variable_frequencies.entry(value).or_default() += 1;
    }
}
