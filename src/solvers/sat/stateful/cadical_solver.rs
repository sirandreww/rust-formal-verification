// ************************************************************************************************
// use
// ************************************************************************************************

use crate::{
    formulas::{Clause, Cube, Literal, CNF},
    solvers::sat::{Assignment, SatResponse},
};

use super::StatefulSatSolver;

// ************************************************************************************************
// struct
// ************************************************************************************************

// #[derive(Default, Clone, Copy)]
pub struct CaDiCalSolver {
    solver: cadical::Solver,
    cadical_literals: Vec<i32>,
    new_literal: i32,
}

// ************************************************************************************************
// impl SplrSolver
// ************************************************************************************************

impl CaDiCalSolver {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn new_lit(&mut self) -> i32 {
        // get new literal not already in the sat solver.
        let result = self.new_literal;
        self.new_literal += 1;
        result
    }

    fn literal_to_cadical_literal(&self, literal: &Literal) -> i32 {
        let index: usize = literal.get_number().try_into().unwrap();
        let cadical_literal = self.cadical_literals[index];
        if literal.is_negated() {
            -cadical_literal
        } else {
            cadical_literal
        }
    }

    fn translate_clause_into_cadical_clause(&self, clause: &Clause) -> Vec<i32> {
        let mut result = Vec::with_capacity(clause.len());
        for literal in clause.iter() {
            result.push(self.literal_to_cadical_literal(literal));
        }
        result
    }

    fn translate_cube_into_cadical_cube(&self, cube: &Cube) -> Vec<i32> {
        let mut result = Vec::with_capacity(cube.len());
        for literal in cube.iter() {
            result.push(self.literal_to_cadical_literal(literal));
        }
        result
    }

    fn extend_cadical_literals_if_needed(&mut self, cnf: &CNF) {
        let max_lit: usize = cnf.get_max_variable_number().try_into().unwrap();
        if max_lit >= self.cadical_literals.len() {
            // reserve so as to avoid having to copy when extending.
            self.cadical_literals
                .reserve(max_lit - self.cadical_literals.len() + 1);
            while max_lit >= self.cadical_literals.len() {
                let new_literal = self.new_lit();
                self.cadical_literals.push(new_literal);
            }
        }
    }

    fn create_dimacs_assignment_from_cadical_model(&self, cadical_literals: &[i32]) -> Vec<i32> {
        let mut result = Vec::with_capacity(cadical_literals.len());
        for (i, lit) in cadical_literals.iter().enumerate().skip(1) {
            let j: i32 = i.try_into().unwrap();
            result.push(match self.solver.value(lit.to_owned()) {
                Some(v) => {
                    if v {
                        j
                    } else {
                        -j
                    }
                }
                None => j,
            });
        }
        result
    }

    fn solve_under_already_translated_assumptions(&mut self, cadical_cube: &[i32]) -> SatResponse {
        let cadical_literals = &self.cadical_literals;

        let sat_result = self.solver.solve_with(cadical_cube.iter().copied());

        match sat_result {
            Some(value) => {
                if value {
                    // sat
                    let result = self.create_dimacs_assignment_from_cadical_model(cadical_literals);
                    SatResponse::Sat {
                        assignment: Assignment::from_dimacs_assignment(&result),
                    }
                } else {
                    // un sat
                    SatResponse::UnSat
                }
            }
            None => panic!("Sat solver error occurred."),
        }
    }

    fn solve_with_just_cube_assumptions(&mut self, cube: Option<&Cube>) -> SatResponse {
        let v = match cube {
            Some(c) => self.translate_cube_into_cadical_cube(c),
            None => Vec::new(),
        };
        self.solve_under_already_translated_assumptions(&v)
    }

    fn handle_temporary_extra_clause(
        &mut self,
        clause: &Clause,
        cube: Option<&Cube>,
    ) -> SatResponse {
        // get clause as cadical vector of bool
        let mut cadical_clause = self.translate_clause_into_cadical_clause(clause);

        // add a entirely new variable
        let some_new_variable = self.new_lit();

        // add the clause c || !var to minisat
        cadical_clause.push(!some_new_variable);
        self.solver.add_clause(cadical_clause.into_iter());

        // this should be added as assumption so as to not make the clause trivial
        let mut assumptions = Vec::new();
        assumptions.push(some_new_variable);

        // add more assumption if present
        if let Some(c) = cube {
            let mut translated_cube = self.translate_cube_into_cadical_cube(c);
            assumptions.append(&mut translated_cube);
        }

        // call sat solver
        let result = self.solve_under_already_translated_assumptions(&assumptions);

        // cancel the previous clause
        self.solver.add_clause([!some_new_variable].into_iter());

        result
    }

    // ************************************************************************************************
    // API functions
    // ************************************************************************************************

    pub fn add_cnf(&mut self, cnf: &CNF) {
        self.extend_cadical_literals_if_needed(cnf);
        for c in cnf.iter() {
            let cadical_clause = self.translate_clause_into_cadical_clause(c);
            self.solver.add_clause(cadical_clause.into_iter())
        }
    }

    pub fn solve(
        &mut self,
        temporary_extra_cube: Option<&Cube>,
        temporary_extra_clause: Option<&Clause>,
    ) -> SatResponse {
        // just in case temporary contain odd literals.
        if let Some(cube) = temporary_extra_cube {
            self.extend_cadical_literals_if_needed(&cube.to_cnf());
        }
        if let Some(clause) = temporary_extra_clause {
            self.extend_cadical_literals_if_needed(&clause.to_cnf());
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

impl StatefulSatSolver for CaDiCalSolver {
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

impl Default for CaDiCalSolver {
    fn default() -> Self {
        Self {
            solver: cadical::Solver::new(),
            cadical_literals: Vec::new(),
            new_literal: 1,
        }
    }
}
