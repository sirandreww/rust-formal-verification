// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::{Clause, Literal, CNF},
    solvers::sat::{Assignment, SatResponse},
};

use super::StatefulSatSolver;
// use minisat;

// ************************************************************************************************
// struct
// ************************************************************************************************

// #[derive(Default, Clone, Copy)]
pub struct MiniSatSolver {
    solver: minisat::Solver,
    mini_sat_literals: Vec<minisat::Bool>,
}

// ************************************************************************************************
// impl SplrSolver
// ************************************************************************************************

impl MiniSatSolver {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn literal_to_mini_sat_literal(&self, literal: &Literal) -> minisat::Bool {
        let index: usize = literal.get_number().try_into().unwrap();
        let mini_sat_literal = self.mini_sat_literals[index];
        if literal.is_negated() {
            !mini_sat_literal
        } else {
            mini_sat_literal
        }
    }

    fn translate_clause_into_mini_sat_clause(&self, clause: &Clause) -> Vec<minisat::Bool> {
        let mut result = Vec::with_capacity(clause.len());
        for literal in clause.iter() {
            result.push(self.literal_to_mini_sat_literal(literal));
        }
        result
    }

    fn extend_mini_sat_literals_if_needed(&mut self, cnf: &CNF) {
        let max_lit: usize = cnf.get_max_variable_number().try_into().unwrap();
        if max_lit >= self.mini_sat_literals.len() {
            // reserve so as to avoid having to copy when extending.
            self.mini_sat_literals
                .reserve(max_lit - self.mini_sat_literals.len());
            while max_lit >= self.mini_sat_literals.len() {
                self.mini_sat_literals.push(self.solver.new_lit())
            }
        }
    }

    fn create_dimacs_assignment_from_mini_sat_model(
        mini_sat_literals: &[minisat::Bool],
        model: &minisat::Model,
    ) -> Vec<i32> {
        let mut result = Vec::with_capacity(mini_sat_literals.len());
        for (i, lit) in mini_sat_literals.iter().enumerate().skip(1) {
            let j: i32 = i.try_into().unwrap();
            result.push(if model.value(lit) { j } else { -j });
        }
        result
    }

    // ************************************************************************************************
    // API functions
    // ************************************************************************************************

    pub fn add_cnf(&mut self, cnf: &CNF) {
        self.extend_mini_sat_literals_if_needed(cnf);
        for c in cnf.iter() {
            let mini_sat_clause = self.translate_clause_into_mini_sat_clause(c);
            self.solver.add_clause(mini_sat_clause)
        }
    }

    pub fn solve(&mut self) -> SatResponse {
        let mini_sat_literals = &self.mini_sat_literals;
        let sat_result = self.solver.solve();
        match sat_result {
            Ok(model) => {
                let result =
                    Self::create_dimacs_assignment_from_mini_sat_model(mini_sat_literals, &model);
                SatResponse::Sat {
                    assignment: Assignment::from_dimacs_assignment(&result),
                }
            }
            Err(()) => SatResponse::UnSat,
        }
    }

    pub fn solve_under_assumptions(&mut self, assumptions: &crate::formulas::Cube) -> SatResponse {
        assumptions.to_cnf();
        todo!()
    }
}

// ************************************************************************************************
// impl trait
// ************************************************************************************************

impl StatefulSatSolver for MiniSatSolver {
    fn add_cnf(&mut self, cnf: &crate::formulas::CNF) {
        self.add_cnf(cnf)
    }

    fn solve(&mut self) -> SatResponse {
        self.solve()
    }

    fn solve_under_assumptions(&mut self, assumptions: &crate::formulas::Cube) -> SatResponse {
        self.solve_under_assumptions(assumptions)
    }
}

impl Default for MiniSatSolver {
    fn default() -> Self {
        Self {
            solver: minisat::Solver::new(),
            mini_sat_literals: Vec::new(),
        }
    }
}
