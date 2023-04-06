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
use register_allocation::register_allocator::{Map, Set};

/// Computes the variables that are live right before the expression runs. In other words, it computes the variables
/// that are live (needed) to execute this expression (and everything after). It does this by modifying
/// `live_variables`.
///
/// `live_variables` - the variables that are live when the next expression runs, mapped to the type of the variable.
/// `variable_frequencies` - maps variables to the number of times they are referenced. Modified in this function.
/// `params` - the parameters of the current function.
///            Parameters are ignored in `liveness_analysis` (since they live on the stack).
/// `interference_graph` - the interference graph that is being constructed in conflict analysis.
pub fn liveness_analysis<'a>(
    expr: &'a Expr,
    live_variables: &mut Map<&'a String, &'a Type>,
    variable_frequencies: &mut Map<&'a String, usize>,
    params: &Set<&'a String>,
    interference_graph: &mut InterferenceGraph<'a>,
    float_interference_graph: &mut InterferenceGraph<'a>,
) {
    match expr {
        Expr::Direct { expr } => liveness_analysis_direct(expr, live_variables, variable_frequencies, params),
        Expr::Let { id, init_expr } => {
            live_variables.remove(id);

            // For variables that are created but never referenced after. These variables still need to be considered
            // and added to the conflict graph for an assignment.
            if variable_frequencies.get(id).is_none() {
                variable_frequencies.insert(id, 0);
            };

            liveness_analysis(
                init_expr,
                live_variables,
                variable_frequencies,
                params,
                interference_graph,
                float_interference_graph,
            );
        }
        Expr::BinaryExpr { operand_1, operand_2, .. } => {
            liveness_analysis_direct(operand_1, live_variables, variable_frequencies, params);
            liveness_analysis_direct(operand_2, live_variables, variable_frequencies, params);
        }
        Expr::UnaryExpr { operand, .. } => {
            liveness_analysis_direct(operand, live_variables, variable_frequencies, params);
        }
        Expr::TypeCoercion { expr, .. } => {
            liveness_analysis_direct(expr, live_variables, variable_frequencies, params);
        }
        Expr::If { condition, then_block, else_block: Some(else_block) } => {
            // Perform conflict analysis on both branches. This allows variables that are declared in the branch
            // to be in the interference graph with other variables used in that branch. Note that these variables
            // are not in the set after the conflict analysis is over (since it was declared inside the block!).
            let mut then_live_variables = live_variables.clone();
            let mut else_live_variables = live_variables.clone();

            conflict_analysis_block(
                then_block,
                params,
                &mut then_live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );

            conflict_analysis_block(
                else_block,
                params,
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
            liveness_analysis_direct(condition, live_variables, variable_frequencies, params);
        }
        Expr::If { condition, then_block, else_block: None } => {
            conflict_analysis_block(
                then_block,
                params,
                live_variables,
                variable_frequencies,
                interference_graph,
                float_interference_graph,
            );

            liveness_analysis_direct(condition, live_variables, variable_frequencies, params);
        }
        Expr::Call { id: _, args, live_variables: call_live_variables } => {
            // Perform liveness analysis on all args.
            for arg in args {
                liveness_analysis_direct(arg, live_variables, variable_frequencies, params);
            }

            // Everything that is live at this point is live right before the call occurs, so we must save them
            // before the call occurs (that is, if they are stored in registers)
            for id in live_variables.keys() {
                call_live_variables.borrow_mut().insert((*id).to_string());
            }
        }
    }
}

// The same as `liveness_analysis` but for a directs.
fn liveness_analysis_direct<'a>(
    direct: &'a DirectExpr,
    live_variables: &mut Map<&'a String, &'a Type>,
    variable_frequencies: &mut Map<&'a String, usize>,
    params: &Set<&'a String>,
) {
    if let DirectExpr::Id { value, id_type } = direct {
        if params.get(value).is_none() {
            live_variables.insert(value, id_type);

            *variable_frequencies.entry(value).or_default() += 1;
        }
    }
}
