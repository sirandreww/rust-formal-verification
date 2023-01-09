// ************************************************************************************************
// use
// ************************************************************************************************

// use std::collections::HashSet;

use crate::algorithms::formula_logic::get_all_variable_numbers_in_cnf;
use crate::formulas::literal::VariableType;
use crate::formulas::{Clause, Cube, Literal, CNF};
use crate::models::AndInverterGraph;
use crate::solvers::sat::{stateless::VarisatSolver, SatResponse};

use super::FiniteStateTransitionSystem;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn get_literal_from_aig_literal(aig_literal: usize) -> Literal {
        let aig_var_num: VariableType = (aig_literal >> 1).try_into().unwrap();
        Literal::new(aig_var_num).negate_if_true(aig_literal % 2 == 1)
    }

    fn get_cnf_that_describes_wire_values_as_a_function_of_latch_values_for_specific_wires(
        aig: &AndInverterGraph,
        wires: &[usize],
    ) -> CNF {
        let and_info = aig.get_and_information_in_cone_of_influence(wires);

        let mut cnf = CNF::new();
        // encode and gates into formula
        for (lhs, rhs0, rhs1) in and_info {
            // get variable numbers
            let and_output = Self::get_literal_from_aig_literal(lhs);
            assert!(!and_output.is_negated());

            // cannot create variable with number 0.
            if rhs0 == 0 || rhs1 == 0 {
                cnf.add_clause(&Clause::new(&[!and_output]));
            } else if rhs0 == 1 && rhs1 == 1 {
                cnf.add_clause(&Clause::new(&[and_output]));
            } else if rhs0 == 1 {
                let and_input_1 = Self::get_literal_from_aig_literal(rhs1);
                cnf.add_clause(&Clause::new(&[!and_input_1, and_output]));
                cnf.add_clause(&Clause::new(&[and_input_1, !and_output]));
            } else if rhs1 == 1 {
                let and_input_0 = Self::get_literal_from_aig_literal(rhs0);
                cnf.add_clause(&Clause::new(&[!and_input_0, and_output]));
                cnf.add_clause(&Clause::new(&[and_input_0, !and_output]));
            } else {
                let and_input_1 = Self::get_literal_from_aig_literal(rhs1);
                let and_input_0 = Self::get_literal_from_aig_literal(rhs0);
                // lhs = rhs0 ^ rhs1 <=> (lhs -> rhs0 ^ rhs1) ^ (lhs <- rhs0 ^ rhs1)
                // <=> (!lhs \/ (rhs0 ^ rhs1)) ^ (lhs \/ !(rhs0 ^ rhs1))
                // <=> ((!lhs \/ rhs0) ^ (!lhs \/ rhs1)) ^ (lhs \/ !rhs0 \/ !rhs1)
                cnf.add_clause(&Clause::new(&[!and_output, and_input_0]));
                cnf.add_clause(&Clause::new(&[!and_output, and_input_1]));
                cnf.add_clause(&Clause::new(&[and_output, !and_input_0, !and_input_1]));
            }
        }

        cnf
    }

    fn create_initial_cnf(aig: &AndInverterGraph) -> Cube {
        let mut cube_literals = Vec::<Literal>::new();

        let latch_info = aig.get_latch_information();
        for (latch_literal, _, latch_reset) in latch_info {
            // if latch is initialized
            if latch_reset != latch_literal {
                let lit = Self::get_literal_from_aig_literal(latch_literal);
                if latch_reset == 0 {
                    cube_literals.push(!lit);
                } else if latch_reset == 1 {
                    cube_literals.push(lit);
                } else {
                    unreachable!();
                }
            }
        }

        Cube::new(&cube_literals)
    }

    fn create_transition_cnf(aig: &AndInverterGraph, max_variable_number: VariableType) -> CNF {
        // propagate new latch values
        let mut latches_to_wires = CNF::new();
        let mut wires_we_care_about = Vec::new();

        let latch_info = aig.get_latch_information();
        // encode latch updates into formula
        for (latch_literal, latch_input, _) in latch_info {
            debug_assert_eq!(latch_literal % 2, 0);
            let latch_lit_before = Self::get_literal_from_aig_literal(latch_literal);
            let latch_lit_after = Literal::new(latch_lit_before.get_number() + max_variable_number)
                .negate_if_true(latch_lit_before.is_negated());
            wires_we_care_about.push(latch_input);

            if latch_input == 0 {
                latches_to_wires.add_clause(&Clause::new(&[!latch_lit_after]));
            } else if latch_input == 1 {
                latches_to_wires.add_clause(&Clause::new(&[latch_lit_after]));
            } else {
                let latch_input_lit = Self::get_literal_from_aig_literal(latch_input);

                // latch_lit = latch_input_lit <=> (latch_lit -> latch_input_lit) ^ (latch_lit <- latch_input_lit)
                // <=> (!latch_lit \/ latch_input_lit) ^ (latch_lit \/ !latch_input_lit)

                latches_to_wires.add_clause(&Clause::new(&[!latch_lit_after, latch_input_lit]));
                latches_to_wires.add_clause(&Clause::new(&[latch_lit_after, !latch_input_lit]));
            }
        }

        latches_to_wires.append(
            &Self::get_cnf_that_describes_wire_values_as_a_function_of_latch_values_for_specific_wires(
                aig,
                &wires_we_care_about,
            )
        );
        latches_to_wires
    }

    fn create_state_to_safety_translation(
        aig: &AndInverterGraph,
        assume_output_is_bad: bool,
    ) -> CNF {
        let mut important_wires = Vec::new();
        important_wires.append(&mut aig.get_bad_information());

        // this should be empty because constrained problem are not supported as of now.
        important_wires.append(&mut aig.get_constraints_information());

        // this is here because sometimes we consider output to be bad
        if assume_output_is_bad {
            important_wires.append(&mut aig.get_output_information());
        }

        Self::get_cnf_that_describes_wire_values_as_a_function_of_latch_values_for_specific_wires(
            aig,
            &important_wires,
        )
    }

    fn create_unsafety_property(aig: &AndInverterGraph, assume_output_is_bad: bool) -> Clause {
        // take all bad into consideration
        let mut bad_info = aig.get_bad_information();
        if assume_output_is_bad {
            bad_info.append(&mut aig.get_output_information());
        }

        // split to 2 cases, depending on if empty or not.
        if !bad_info.is_empty() {
            let mut unsafe_literals = Vec::new();
            for bad_literal in bad_info {
                let b_lit = Self::get_literal_from_aig_literal(bad_literal);
                unsafe_literals.push(b_lit);
            }
            Clause::new(&unsafe_literals)
        } else {
            // the empty clause is un-sat when turned into cnf
            let result = Clause::new(&[]);
            debug_assert!(
                VarisatSolver::default().solve_cnf(&result.to_cnf()) == SatResponse::UnSat
            );
            // debug_assert!(VarisatSolver::default().solve_cnf(&(!result.to_owned()).to_cnf()) == SatResponse::Sat { assignment: Assignment::from_dimacs_assignment });
            // let p = (!result).to_cnf();
            result
        }
    }

    fn create_input_and_state_literal_numbers(
        aig: &AndInverterGraph,
    ) -> (Vec<VariableType>, Vec<VariableType>) {
        let mut input_literals = Vec::new();
        for input_literal in aig.get_input_information() {
            let lit = Self::get_literal_from_aig_literal(input_literal);
            assert!(!lit.is_negated());
            input_literals.push(lit.get_number());
        }

        let mut state_literals = Vec::new();
        for (latch_literal, _, _) in aig.get_latch_information() {
            let lit = Self::get_literal_from_aig_literal(latch_literal);
            assert!(!lit.is_negated());
            state_literals.push(lit.get_number());
        }
        (input_literals, state_literals)
    }

    // ********************************************************************************************
    // aig api functions
    // ********************************************************************************************

    /// Function that converts an AndInverterGraph into a FiniteStateTransitionSystem.
    ///
    /// # Arguments
    ///
    /// * `aig: &AndInverterGraph` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig, false);
    /// assert_eq!(fsts.get_initial_relation().to_string(), "p cnf 3 3\n-1 0\n-2 0\n-3 0");
    /// ```
    pub fn from_aig(aig: &AndInverterGraph, assume_output_is_bad: bool) -> Self {
        // perform some checks first
        let max_variable_number_as_usize = aig.get_highest_variable_number();
        assert!(
            max_variable_number_as_usize < (u32::MAX >> 1).try_into().unwrap(),
            "AIG has variables with numbers that are too high (too many variables)."
        );
        assert!(
            aig.get_constraints_information().is_empty(),
            "Making 'FiniteStateTransitionSystem' from aig with constraints is not supported.\nTry folding the AIG with another tool First"
        );

        // make formulas
        let max_literal_number: VariableType = max_variable_number_as_usize.try_into().unwrap();
        let (input_literals, state_literals) = Self::create_input_and_state_literal_numbers(aig);
        let initial_states: Cube = Self::create_initial_cnf(aig);
        let transition: CNF = Self::create_transition_cnf(aig, max_literal_number);
        let state_to_safety_translation: CNF =
            Self::create_state_to_safety_translation(aig, assume_output_is_bad);
        let unsafety_property: Clause = Self::create_unsafety_property(aig, assume_output_is_bad);
        let initial_literals = initial_states.iter().map(|l| l.to_owned()).collect();

        let cone_of_safety = get_all_variable_numbers_in_cnf(&state_to_safety_translation);
        let cone_of_transition = get_all_variable_numbers_in_cnf(&transition);

        // let cone_of_safety_only_latches: HashSet<VariableType> = cone_of_safety.iter().filter(|v| input_literals.contains(v)).map(|v| v.to_owned()).collect();
        // let cone_of_transition_only_latches: HashSet<VariableType> = cone_of_transition.iter().filter(|v| input_literals.contains(v)).map(|v| v.to_owned()).collect();

        // create object
        Self {
            initial_literals,
            initial_states,
            transition,
            state_to_safety_translation,
            unsafety_property,
            max_literal_number,
            state_literals,
            input_literals,
            cone_of_safety,
            cone_of_transition,
            // cone_of_safety_only_latches,
            // cone_of_transition_only_latches
        }
    }
}
