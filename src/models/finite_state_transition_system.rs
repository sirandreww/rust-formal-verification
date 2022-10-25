// ************************************************************************************************
// use
// ************************************************************************************************

use crate::formulas::{Clause, Literal, Variable, CNF};
use crate::models::AndInverterGraph;

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct FiniteStateTransitionSystem {
    initial_states: CNF,
    transition: CNF,
    safety_property: CNF,
    unsafety_property: CNF,
    max_variable_number: u32,
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
        safety_property: CNF,
        unsafety_property: CNF,
        max_variable_number: u32,
    ) -> Self {
        // debug_assert!(
        //     initial_states.get_greatest_variable_number() <= number_of_variables,
        //     "initial_states has variable with a higher number than number_of_variables."
        // );
        // debug_assert!(
        //     transition.get_highest_variable_number() <= max_variable_number,
        //     "transition has variable with a higher number than number_of_variables."
        // );
        Self {
            initial_states,
            transition,
            safety_property,
            unsafety_property,
            max_variable_number,
        }
    }

    fn get_literal_from_aig_literal(aig_literal: usize) -> Literal {
        let aig_var_num: u32 = (aig_literal >> 1).try_into().unwrap();
        Literal::new_with_negation_option(&Variable::new(aig_var_num), aig_literal % 2 == 1)
    }

    fn propagate_latch_values(
        aig: &AndInverterGraph,
        cnf_to_add_to: &mut CNF,
        depth: u32,
        max_variable_number: u32,
    ) {
        let and_info = aig.get_and_information();
        let mut cnf = CNF::default();
        // encode and gates into formula
        for (lhs, rhs0, rhs1) in and_info {
            // get variable numbers
            let and_output = Self::get_literal_from_aig_literal(lhs);
            assert_eq!(and_output.is_negated(), false);

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

        Self::bump_all_cnf_variables_by_some_number(
            &cnf,
            depth * max_variable_number,
            cnf_to_add_to,
        );
    }

    fn create_initial_cnf(aig: &AndInverterGraph, max_variable_number: u32) -> CNF {
        let mut cnf = CNF::default();

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

        // todo!("Change this");
        // propagate latch values to all other wires in circuit for first clk.
        Self::propagate_latch_values(aig, &mut cnf, 0, max_variable_number);

        cnf
    }

    fn create_transition_cnf(aig: &AndInverterGraph, max_variable_number: u32) -> CNF {
        let mut cnf = CNF::default();

        let latch_info = aig.get_latch_information();
        // encode latch updates into formula
        for (latch_literal, latch_input, _) in latch_info {
            assert_eq!(latch_literal % 2, 0);
            let latch_lit_before = Self::get_literal_from_aig_literal(latch_literal);
            let latch_lit_after = Literal::new_with_negation_option(
                &Variable::new(latch_lit_before.get_number() + max_variable_number),
                latch_lit_before.is_negated(),
            );
            if latch_input == 0 {
                cnf.add_clause(&Clause::new(&[!latch_lit_after]));
            } else if latch_input == 1 {
                cnf.add_clause(&Clause::new(&[latch_lit_after]));
            } else {
                let latch_input_lit = Self::get_literal_from_aig_literal(latch_input);

                // latch_lit = latch_input_lit <=> (latch_lit -> latch_input_lit) ^ (latch_lit <- latch_input_lit)
                // <=> (!latch_lit \/ latch_input_lit) ^ (latch_lit \/ !latch_input_lit)
    
                cnf.add_clause(&Clause::new(&[!latch_lit_after, latch_input_lit]));
                cnf.add_clause(&Clause::new(&[latch_lit_after, !latch_input_lit]));
            }
        }

        // propagate new latch values
        Self::propagate_latch_values(aig, &mut cnf, 1, max_variable_number);

        cnf
    }

    fn create_safety_property(aig: &AndInverterGraph) -> CNF {
        let mut cnf = CNF::default();
        let bad_info = aig.get_bad_information();
        for bad_literal in bad_info {
            let var_num: u32 = (bad_literal >> 1).try_into().unwrap();
            let b_lit =
                Literal::new_with_negation_option(&Variable::new(var_num), bad_literal % 2 == 1);
            cnf.add_clause(&Clause::new(&[!b_lit]));
        }
        cnf
    }

    fn create_unsafety_property(aig: &AndInverterGraph) -> CNF {
        let mut clause = Clause::new(&[]);
        let bad_info = aig.get_bad_information();
        for bad_literal in bad_info {
            let var_num: u32 = (bad_literal >> 1).try_into().unwrap();
            let b_lit =
                Literal::new_with_negation_option(&Variable::new(var_num), bad_literal % 2 == 1);
            clause.add_literal(&b_lit);
        }
        let mut cnf = CNF::default();
        if !clause.is_empty() {
            cnf.add_clause(&clause);
        }
        cnf
    }

    fn bump_all_cnf_variables_by_some_number(
        original_cnf: &CNF,
        number_to_bump: u32,
        cnf_to_add_to: &mut CNF,
    ) {
        if number_to_bump == 0 {
            // this makes the function faster for the simple case
            cnf_to_add_to.concat(original_cnf);
        } else {
            for clause in original_cnf.iter() {
                let mut new_clause = Clause::new(&[]);
                for literal in clause.iter() {
                    let mut new_number = literal.get_number();
                    assert_ne!(literal.get_number(), 0);
                    new_number += number_to_bump;
                    let is_negated = literal.is_negated();
                    let new_lit = Literal::new_with_negation_option(
                        &Variable::new(new_number), is_negated
                    );
                    new_clause.add_literal(&new_lit);
                }
                cnf_to_add_to.add_clause(&new_clause);
            }
        }
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
    /// let file_path = "tests/simple_examples/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// assert_eq!(fsts.get_initial_states().to_string(), "((!x1) & (!x2) & (!x3))");
    /// ```
    pub fn from_aig(aig: &AndInverterGraph) -> Self {
        let max_variable_number_as_usize = aig.get_maximum_variable_number();
        assert!(
            max_variable_number_as_usize < (u32::MAX >> 1).try_into().unwrap(),
            "AIG has variables with numbers that are too high"
        );
        let max_variable_number: u32 = max_variable_number_as_usize.try_into().unwrap();
        let initial_states: CNF = Self::create_initial_cnf(aig, max_variable_number);
        let transition: CNF = Self::create_transition_cnf(aig, max_variable_number);
        let safety_property: CNF = Self::create_safety_property(aig);
        let unsafety_property: CNF = Self::create_unsafety_property(aig);
        Self::new(
            initial_states,
            transition,
            safety_property,
            unsafety_property,
            max_variable_number,
        )
    }

    pub fn get_initial_relation(&self, cnf_to_add_to: &mut CNF) {
        cnf_to_add_to.concat(&self.initial_states);
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
    /// let file_path = "tests/simple_examples/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// let fsts = FiniteStateTransitionSystem::from_aig(&aig);
    /// let mut tr_x_x_tag = CNF::default();
    /// fsts.get_transition_relation_for_some_depth(1, &mut tr_x_x_tag);
    /// assert_eq!(
    ///     tr_x_x_tag.to_string(), 
    ///     "((!x1 | x7) & (!x2 | x8) & (!x5 | x6) & (!x6 | !x10) & (!x7 | !x9) & (!x8 | !x9) & (x1 | !x7) & (x2 | !x8) & (x5 | !x6) & (x6 | !x9 | x10) & (x7 | x8 | x9) & (x9 | !x10))"
    /// );
    /// ```
    pub fn get_transition_relation_for_some_depth(&self, depth: u32, cnf_to_add_to: &mut CNF) {
        debug_assert!(depth > 0, "Called get_transition_relation_for_some_depth with depth 0. Transition relation for depth 0 is undefined.");
        Self::bump_all_cnf_variables_by_some_number(
            &self.transition,
            self.max_variable_number * (depth - 1),
            cnf_to_add_to,
        );
    }

    pub fn get_unsafety_property_for_some_depth(&self, depth: u32, cnf_to_add_to: &mut CNF) {
        Self::bump_all_cnf_variables_by_some_number(
            &self.unsafety_property,
            self.max_variable_number * depth,
            cnf_to_add_to,
        );
    }

    pub fn get_safety_property_for_some_depth(&self, depth: u32, cnf_to_add_to: &mut CNF) {
        Self::bump_all_cnf_variables_by_some_number(
            &self.safety_property,
            self.max_variable_number * depth,
            cnf_to_add_to,
        );
    }

    // pub fn Transition(&self) -> CNF {}
}
