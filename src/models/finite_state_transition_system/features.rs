// ************************************************************************************************
// use
// ************************************************************************************************

use crate::algorithms::formula_logic::{does_a_imply_b, is_a_and_b_satisfiable};
use crate::formulas::literal::VariableType;
use crate::formulas::{Clause, Cube, Literal, CNF};
use crate::solvers::sat::{Assignment, StatelessSatSolver};

use super::FiniteStateTransitionSystem;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bump_all_clause_variables_by_some_number(
        original_clause: &Clause,
        number_to_bump: VariableType,
    ) -> Clause {
        let mut literals = Vec::new();
        for literal in original_clause.iter() {
            assert_ne!(literal.get_number(), 0);
            let mut new_number = literal.get_number();
            new_number += number_to_bump;
            let is_negated = literal.is_negated();
            let new_lit = Literal::new(new_number).negate_if_true(is_negated);
            literals.push(new_lit);
        }
        Clause::new(&literals)
    }

    fn bump_all_cnf_variables_by_some_number(
        original_cnf: &CNF,
        number_to_bump: VariableType,
    ) -> CNF {
        if number_to_bump == 0 {
            // this makes the function faster for the simple case
            original_cnf.to_owned()
        } else {
            let mut cnf_to_add_to = CNF::new();
            for clause in original_cnf.iter() {
                let new_clause =
                    Self::bump_all_clause_variables_by_some_number(clause, number_to_bump);
                cnf_to_add_to.add_clause(&new_clause);
            }
            cnf_to_add_to
        }
    }

    // ********************************************************************************************
    // aig api functions
    // ********************************************************************************************

    pub fn add_tags_to_relation(&self, relation: &CNF, number_of_tags: VariableType) -> CNF {
        Self::bump_all_cnf_variables_by_some_number(
            relation,
            self.max_literal_number * number_of_tags,
        )
    }

    pub fn add_tags_to_cube(&self, cube: &Cube, number_of_tags: VariableType) -> Cube {
        if number_of_tags == 0 {
            // this makes the function faster for the simple case
            cube.to_owned()
        } else {
            let bumped_as_clause = Self::bump_all_clause_variables_by_some_number(
                &(!(cube.to_owned())),
                self.max_literal_number * number_of_tags,
            );
            !bumped_as_clause
        }
    }

    pub fn add_tags_to_clause(&self, clause: &Clause, number_of_tags: VariableType) -> Clause {
        if number_of_tags == 0 {
            // this makes the function faster for the simple case
            clause.to_owned()
        } else {
            Self::bump_all_clause_variables_by_some_number(
                clause,
                self.max_literal_number * number_of_tags,
            )
        }
    }

    pub fn is_cube_initial(&self, cube: &Cube) -> bool {
        // check that cube contains no contradiction with initial.
        for literal in cube.iter() {
            if self.initial_literals.contains(&!literal.to_owned()) {
                return false;
            }
        }
        true
    }

    pub fn extract_state_from_assignment(&self, assignment: &Assignment) -> Cube {
        let mut literals = Vec::new();

        for state_lit_num in &self.state_literals {
            literals.push(
                Literal::new(state_lit_num.to_owned())
                    .negate_if_true(!assignment.get_value(state_lit_num).unwrap()),
            )
        }

        Cube::new(&literals)
    }

    pub fn intersect_cube_with_cone_of_safety(&self, c: &Cube) -> Cube {
        let filtered_c: Vec<Literal> = c
            .iter()
            .filter(|l| self.cone_of_safety_only_latches.contains(&l.get_number()))
            .map(|l| l.to_owned())
            .collect();
        Cube::new(&filtered_c)
    }

    pub fn intersect_cube_with_cone_of_transition(&self, c: &Cube) -> Cube {
        let filtered_c: Vec<Literal> = c
            .iter()
            .filter(|l| {
                self.cone_of_transition_only_latches
                    .contains(&l.get_number())
            })
            .map(|l| l.to_owned())
            .collect();
        Cube::new(&filtered_c)
    }

    // pub fn get_clone_of_cube(&self, c: &Cube) -> HashSet<VariableType> {
    //     let mut set = HashSet::new();
    //     set.reserve(self.state_literals.len());
    //     for l in c.iter() {
    //         let current_clone = self.cones_of_state_literals.get(&l.get_number()).unwrap();
    //         for v in current_clone {
    //             set.insert(v.to_owned());
    //         }
    //     }
    //     set
    // }

    // pub fn intersect_cube_with_clone_of_other_cube(&self, c: &Cube, other: &Cube) -> Cube {
    //     let cone_of_other = self.get_clone_of_cube(other);

    //     let filtered_c: Vec<Literal> = c
    //         .iter()
    //         .filter(|l| cone_of_other.contains(&l.get_number()))
    //         .map(|l| l.to_owned())
    //         .collect();
    //     Cube::new(&filtered_c)
    // }

    pub fn check_invariant<T: StatelessSatSolver>(&self, inv_candidate: &CNF) {
        // println!("inv_candidate = {}", inv_candidate);
        // check INIT -> inv_candidate
        let mut init = self.get_initial_relation().to_cnf();
        init.append(&self.get_state_to_safety_translation());
        // println!("init = {}", init);

        assert!(
            does_a_imply_b::<T>(&init, inv_candidate),
            "Invariant does not cover all of init."
        );

        // check inv_candidate && Tr -> inv_candidate'
        let mut a = self.get_transition_relation();
        a.append(inv_candidate);
        a.append(&self.get_state_to_safety_translation());
        a.append(&self.add_tags_to_relation(&self.get_state_to_safety_translation(), 1));
        let b = self.add_tags_to_relation(inv_candidate, 1);
        assert!(
            does_a_imply_b::<T>(&a, &b),
            "Invariant doesn't cover all of the reachable states."
        );

        // check inv_candidate ^ !p is un-sat
        let mut bad = self.get_unsafety_property().to_cnf();
        bad.append(&self.get_state_to_safety_translation());
        assert!(
            !is_a_and_b_satisfiable::<T>(inv_candidate, &bad),
            "Invariant isn't always safe.",
        );
    }
}
