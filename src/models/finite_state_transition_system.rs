// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::literal::VariableType;
use crate::formulas::{Clause, Literal, CNF};
use crate::models::AndInverterGraph;
use crate::solvers::sat::{VarisatSolver, SatResponse};

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Clone)]
pub struct FiniteStateTransitionSystem {
    initial_states: CNF,
    transition: CNF,
    state_and_property_connection: CNF,
    unsafety_property: Clause,
    max_literal_number: VariableType,
    state_literals: Vec<VariableType>,
    input_literals: Vec<VariableType>,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl FiniteStateTransitionSystem {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn new(
        initial_states: CNF,
        transition: CNF,
        state_and_property_connection: CNF, 
        unsafety_property: Clause,
        max_literal_number: VariableType,
        state_literals: Vec<VariableType>,
        input_literals: Vec<VariableType>,
    ) -> Self {
        Self {
            initial_states,
            transition,
            state_and_property_connection,
            unsafety_property,
            max_literal_number,
            state_literals,
            input_literals,
        }
    }

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

    fn create_initial_cnf(aig: &AndInverterGraph) -> CNF {
        let mut cnf = CNF::new();

        let latch_info = aig.get_latch_information();
        for (latch_literal, _, latch_reset) in latch_info {
            // if latch is initialized
            if latch_reset != latch_literal {
                let lit = Self::get_literal_from_aig_literal(latch_literal);
                if latch_reset == 0 {
                    cnf.add_clause(&Clause::new(&[!lit]));
                } else if latch_reset == 1 {
                    cnf.add_clause(&Clause::new(&[lit]));
                } else {
                    unreachable!();
                }
            }
        }
        cnf
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

    // fn create_safety_property(aig: &AndInverterGraph) -> CNF {
    //     let bad_info = aig.get_bad_information();
    //     if !bad_info.is_empty() {
    //         let mut latches_to_wires = Self::get_cnf_that_describes_wire_values_as_a_function_of_latch_values_for_specific_wires(
    //             aig,
    //             &bad_info,
    //         );
    //         for bad_literal in bad_info {
    //             let b_lit = Self::get_literal_from_aig_literal(bad_literal);
    //             latches_to_wires.add_clause(&Clause::new(&[!b_lit]));
    //         }
    //         latches_to_wires
    //     } else {
    //         CNF::new()
    //     }
    // }

    fn create_state_and_property_connection(aig: &AndInverterGraph) -> CNF {
        let mut important_wires = Vec::new();
        important_wires.append(& mut aig.get_bad_information());
        important_wires.append(& mut aig.get_constraints_information());
        important_wires.append(& mut aig.get_output_information());

        Self::get_cnf_that_describes_wire_values_as_a_function_of_latch_values_for_specific_wires(
            aig,
            &important_wires,
        )
    }

    fn create_unsafety_property(aig: &AndInverterGraph) -> Clause {
        let bad_info = aig.get_bad_information();
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
            let not_p = result.to_cnf();
            debug_assert!(VarisatSolver::default().solve_cnf(&not_p) == SatResponse::UnSat);
            // let p = (!result).to_cnf();
            result
        }
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
                let mut literals = Vec::new();
                for literal in clause.iter() {
                    assert_ne!(literal.get_number(), 0);
                    let mut new_number = literal.get_number();
                    new_number += number_to_bump;
                    let is_negated = literal.is_negated();
                    let new_lit = Literal::new(new_number).negate_if_true(is_negated);
                    literals.push(new_lit);
                }

                let new_clause = Clause::new(&literals);
                cnf_to_add_to.add_clause(&new_clause);
            }
            cnf_to_add_to
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
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// assert_eq!(fsts.get_initial_relation().to_string(), "((!x1) & (!x2) & (!x3))");
    /// ```
    pub fn from_aig(aig: &AndInverterGraph) -> Self {
        let max_variable_number_as_usize = aig.get_highest_variable_number();
        assert!(
            max_variable_number_as_usize < (u32::MAX >> 1).try_into().unwrap(),
            "AIG has variables with numbers that are too high (too many variables)."
        );
        if !aig.get_constraints_information().is_empty() {
            // eprintln!("Warning: Making 'FiniteStateTransitionSystem' from aig with constraints, constraints are not yet supported, they will simply be ignored.");
            println!("Warning: Making 'FiniteStateTransitionSystem' from aig with constraints, constraints are not yet supported, they will simply be ignored.");
        }
        let max_literal_number: VariableType = max_variable_number_as_usize.try_into().unwrap();
        let (input_literals, state_literals) = Self::create_input_and_state_literal_numbers(aig);
        let initial_states: CNF = Self::create_initial_cnf(aig);
        let transition: CNF = Self::create_transition_cnf(aig, max_literal_number);
        // let safety_property: CNF = Self::create_safety_property(aig);
        let state_and_property_connection: CNF = Self::create_state_and_property_connection(aig);
        let unsafety_property: Clause = Self::create_unsafety_property(aig);
        Self::new(
            initial_states,
            transition,
            state_and_property_connection,
            unsafety_property,
            max_literal_number,
            state_literals,
            input_literals,
        )
    }

    pub fn get_max_literal_number(&self) -> VariableType {
        self.max_literal_number
    }

    pub fn get_state_literal_numbers(&self) -> Vec<VariableType> {
        self.state_literals.to_owned()
    }

    pub fn get_input_literal_numbers(&self) -> Vec<VariableType> {
        self.input_literals.to_owned()
    }

    pub fn get_initial_relation(&self) -> CNF {
        self.initial_states.to_owned()
    }

    /// Function that gets the transition relation for the FiniteStateTransitionSystem.
    ///
    /// # Arguments
    ///
    /// * `self: &FiniteStateTransitionSystem` - the FiniteStateTransitionSystem desired.
    /// * `depth: u32` - the number of tags desire, for example for Tr(X, X') this would be 1
    ///                  for Tr(X', X'') this would be 2 and so on.
    /// * `cnf_to_add_to: &mut CNF` - cnf that the result would be added to for performance reasons.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::{AndInverterGraph, FiniteStateTransitionSystem};
    /// use rust_formal_verification::formulas::CNF;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// let mut tr_x_x_tag = CNF::default();
    /// let tr_x_x_tag = fsts.get_transition_relation_for_some_depth(1);
    /// assert_eq!(
    ///     tr_x_x_tag.to_string(),
    ///     "((x1 | !x7) & (!x1 | !x5) & (!x1 | x7) & (x2 | !x8) & (!x2 | !x4) & (!x2 | x8) & (!x3 | !x4) & (x4 | !x5) & (x5 | !x6) & (!x5 | x6) & (x1 | !x4 | x5) & (x2 | x3 | x4))"
    /// );
    /// ```
    pub fn get_transition_relation(&self) -> CNF {
        self.transition.to_owned()
    }

    pub fn get_state_and_property_connection_relation(&self) -> CNF {
        self.state_and_property_connection.to_owned()
    }

    pub fn get_safety_property(&self) -> CNF {
        (!self.unsafety_property.to_owned()).to_cnf()
    }

    pub fn get_unsafety_property(&self) -> CNF {
        self.unsafety_property.to_cnf()
    }

    pub fn add_tags_to_relation(&self, relation: &CNF, number_of_tags: VariableType) -> CNF {
        Self::bump_all_cnf_variables_by_some_number(relation, self.max_literal_number * number_of_tags)
    }
}
