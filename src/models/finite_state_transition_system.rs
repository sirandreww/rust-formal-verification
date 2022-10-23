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
    max_variable_number: i32,
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
        max_variable_number: i32,
    ) -> Self {
        // debug_assert!(
        //     initial_states.get_greatest_variable_number() <= number_of_variables,
        //     "initial_states has variable with a higher number than number_of_variables."
        // );
        // debug_assert!(
        //     transition.get_greatest_variable_number() <= number_of_variables,
        //     "transition has variable with a higher number than number_of_variables."
        // );
        Self {
            initial_states: initial_states,
            transition: transition,
            safety_property: safety_property,
            unsafety_property: unsafety_property,
            max_variable_number: max_variable_number,
        }
    }

    fn create_initial_cnf(aig: &AndInverterGraph) -> CNF {
        let latch_info = aig.get_latch_information();
        let mut cnf = CNF::default();
        for (latch_literal, _, latch_reset) in latch_info {
            // if latch is initialized
            if latch_reset != (latch_literal >> 1) {
                let var_num: i32 = (latch_literal >> 1).try_into().unwrap();
                let lit = Literal::new(&Variable::new(var_num), false);
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

    fn create_transition_cnf(aig: &AndInverterGraph, max_variable_number: i32) -> CNF {
        let mut cnf = CNF::default();
        let and_info = aig.get_and_information();

        // encode and gates into formula
        for (lhs, rhs0, rhs1) in and_info {
            let lhs_num: i32 = (lhs >> 1).try_into().unwrap();
            let rhs0_num: i32 = (rhs0 >> 1).try_into().unwrap();
            let rhs1_num: i32 = (rhs1 >> 1).try_into().unwrap();

            assert_eq!(lhs % 2, 0);
            let lhs_lit = Literal::new(&Variable::new(lhs_num), lhs % 2 == 1);
            let rhs0_lit = Literal::new(&Variable::new(rhs0_num), rhs0 % 2 == 1);
            let rhs1_lit = Literal::new(&Variable::new(rhs1_num), rhs1 % 2 == 1);

            // lhs = rhs0 ^ rhs1 <=> (lhs -> rhs0 ^ rhs1) ^ (lhs <- rhs0 ^ rhs1)
            // <=> (!lhs \/ (rhs0 ^ rhs1)) ^ (lhs \/ !(rhs0 ^ rhs1))
            // <=> ((!lhs \/ rhs0) ^ (!lhs \/ rhs1)) ^ (lhs \/ !rhs0 \/ !rhs1)
            cnf.add_clause(&Clause::new(&[!lhs_lit, rhs0_lit]));
            cnf.add_clause(&Clause::new(&[!lhs_lit, rhs1_lit]));
            cnf.add_clause(&Clause::new(&[lhs_lit, !rhs0_lit, !rhs1_lit]));
        }

        let latch_info = aig.get_latch_information();
        // encode latch updates into formula
        for (latch_literal, latch_input, _) in latch_info {
            let var_num: i32 = (latch_literal >> 1).try_into().unwrap();
            let var_num_after_transition = max_variable_number + var_num;
            let input_var_num: i32 = (latch_input >> 1).try_into().unwrap();

            assert_eq!(latch_literal % 2, 0);
            let latch_lit = Literal::new(
                &Variable::new(var_num_after_transition),
                latch_literal % 2 == 1,
            );
            let latch_input_lit = Literal::new(&Variable::new(input_var_num), latch_input % 2 == 1);

            // latch_lit = latch_input_lit <=> (latch_lit -> latch_input_lit) ^ (latch_lit <- latch_input_lit)
            // <=> (!latch_lit \/ latch_input_lit) ^ (latch_lit \/ !latch_input_lit)

            cnf.add_clause(&Clause::new(&[!latch_lit, latch_input_lit]));
            cnf.add_clause(&Clause::new(&[latch_lit, !latch_input_lit]));
        }
        cnf
    }

    fn create_safety_property(aig: &AndInverterGraph) -> CNF {
        let mut cnf = CNF::default();
        let bad_info = aig.get_bad_information();
        for bad_literal in bad_info {
            let var_num: i32 = (bad_literal >> 1).try_into().unwrap();
            let b_lit = Literal::new(&Variable::new(var_num), bad_literal % 2 == 1);
            cnf.add_clause(&Clause::new(&[!b_lit]));
        }
        cnf
    }

    fn create_unsafety_property(aig: &AndInverterGraph) -> CNF {
        let mut clause = Clause::new(&[]);
        let bad_info = aig.get_bad_information();
        for bad_literal in bad_info {
            let var_num: i32 = (bad_literal >> 1).try_into().unwrap();
            let b_lit = Literal::new(&Variable::new(var_num), bad_literal % 2 == 1);
            clause.add_literal(&b_lit);
        }
        let mut cnf = CNF::default();
        if !clause.is_empty() {
            cnf.add_clause(&clause);
        }
        cnf
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
            max_variable_number_as_usize < (i32::MAX).try_into().unwrap(),
            "AIG has variables with numbers that are too high"
        );
        let max_variable_number: i32 = max_variable_number_as_usize.try_into().unwrap();
        let initial_states: CNF = Self::create_initial_cnf(aig);
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

    pub fn get_initial_states(&self) -> &CNF {
        &self.initial_states
    }

    pub fn get_transition_formula(&self) -> &CNF {
        &self.transition
    }

    pub fn get_safety_property(&self) -> &CNF {
        &self.safety_property
    }

    pub fn get_unsafety_property(&self) -> &CNF {
        &self.unsafety_property
    }

    // pub fn Transition(&self) -> CNF {}
}
