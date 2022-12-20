// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::{Clause, Cube, Literal, CNF},
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

    fn translate_cube_into_mini_sat_cube(&self, cube: &Cube) -> Vec<minisat::Bool> {
        let mut result = Vec::with_capacity(cube.len());
        for literal in cube.iter() {
            result.push(self.literal_to_mini_sat_literal(literal));
        }
        result
    }

    fn extend_mini_sat_literals_if_needed(&mut self, cnf: &CNF) {
        let max_lit: usize = cnf.get_max_variable_number().try_into().unwrap();
        if max_lit >= self.mini_sat_literals.len() {
            // reserve so as to avoid having to copy when extending.
            self.mini_sat_literals
                .reserve(max_lit - self.mini_sat_literals.len() + 1);
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

    fn solve_under_already_translated_assumptions(
        &mut self,
        mini_sat_cube: &[minisat::Bool],
    ) -> SatResponse {
        let mini_sat_literals = &self.mini_sat_literals;

        let sat_result = self
            .solver
            .solve_under_assumptions(mini_sat_cube.to_owned());

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

    fn solve_with_just_cube_assumptions(&mut self, cube: Option<&Cube>) -> SatResponse {
        let v = match cube {
            Some(c) => self.translate_cube_into_mini_sat_cube(c),
            None => Vec::new(),
        };
        self.solve_under_already_translated_assumptions(&v)
    }

    fn handle_temporary_extra_clause(
        &mut self,
        clause: &Clause,
        cube: Option<&Cube>,
    ) -> SatResponse {
        // get clause as mini sat vector of bool
        let mut mini_sat_clause = self.translate_clause_into_mini_sat_clause(clause);

        // add a entirely new variable
        let some_new_variable = self.solver.new_lit();

        // add the clause c || !var to minisat
        mini_sat_clause.push(!some_new_variable);
        self.solver.add_clause(mini_sat_clause);

        // this should be added as assumption so as to not make the clause trivial
        let mut assumptions = Vec::new();
        assumptions.push(some_new_variable);

        // add more assumption if present
        if let Some(c) = cube {
            let mut translated_cube = self.translate_cube_into_mini_sat_cube(c);
            assumptions.append(&mut translated_cube);
        }

        // call sat solver
        let result = self.solve_under_already_translated_assumptions(&assumptions);

        // cancel the previous clause
        self.solver.add_clause([!some_new_variable]);

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

    pub fn solve(
        &mut self,
        temporary_extra_cube: Option<&Cube>,
        temporary_extra_clause: Option<&Clause>,
    ) -> SatResponse {
        // just in case temporary contain odd literals.
        if let Some(cube) = temporary_extra_cube {
            self.extend_mini_sat_literals_if_needed(&cube.to_cnf());
        }
        if let Some(clause) = temporary_extra_clause {
            self.extend_mini_sat_literals_if_needed(&clause.to_cnf());
        }

        match temporary_extra_clause {
            None => self.solve_with_just_cube_assumptions(temporary_extra_cube),
            Some(extra_clause) => {
                self.handle_temporary_extra_clause(extra_clause, temporary_extra_cube)
            }
        }
        // if temporary_extra_cube == None && temporary_extra_clause == None
        // assumptions.to_cnf();
        // todo!()
    }
}

// ************************************************************************************************
// impl trait
// ************************************************************************************************

impl StatefulSatSolver for MiniSatSolver {
    fn add_cnf(&mut self, cnf: &crate::formulas::CNF) {
        self.add_cnf(cnf)
    }

    fn solve(
        &mut self,
        temporary_extra_cube: Option<&Cube>,
        temporary_extra_clause: Option<&Clause>,
    ) -> SatResponse {
        self.solve(temporary_extra_cube, temporary_extra_clause)
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
