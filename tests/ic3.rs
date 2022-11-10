// ************************************************************************************************
// mod declaration
// ************************************************************************************************

mod common;

// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::{
        algorithms::{ic3::IC3Result, IC3, formula_logic::does_a_imply_b},
        formulas::CNF,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{CadicalSolver, SatSolver, SplrSolver, VarisatSolver},
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn check_invariant<T: SatSolver>(fin_state: &FiniteStateTransitionSystem, inv_candidate: &CNF) {
        // check INIT -> inv_candidate
        let init = fin_state.get_initial_relation();
        assert!(does_a_imply_b::<T>(&init, inv_candidate), "Invariant does not cover all of init.");

        // check inv_candidate && Tr -> inv_candidate'
        let mut a = fin_state.get_transition_relation();
        a.append(inv_candidate);
        let b = fin_state.add_tags_to_relation(inv_candidate, 1);
        assert!(does_a_imply_b::<T>(&a, &b), "Invariant doesn't cover all of the reachable states.");

        // check inv_candidate -> p
        assert!(does_a_imply_b::<T>(inv_candidate, &fin_state.get_safety_property()), "Invariant isn't always safe.",);

        println!("Invariant check passed!");
    }

    fn test_ic3<T: SatSolver>(fin_state: &FiniteStateTransitionSystem, _aig: &AndInverterGraph) {
        let mut ic3_solver = IC3::<T>::new(fin_state);
        match ic3_solver.prove() {
            IC3Result::Proof { invariant } => {
                println!("Safe, checking invariant.");
                check_invariant::<T>(fin_state, &invariant);
            }
            IC3Result::CTX { depth } => {
                // do nothing for now
                println!("Unsafe, depth = {}", depth);
            }
        }
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn ic3_on_our_examples() {
        let file_paths = common::_get_paths_to_all_our_example_aig_files();
        for aig_file_path in file_paths {
            println!("file_path = {}", aig_file_path);

            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            test_ic3::<SplrSolver>(&fin_state, &aig);
            test_ic3::<VarisatSolver>(&fin_state, &aig);
            test_ic3::<CadicalSolver>(&fin_state, &aig);
        }
    }

    #[test]
    fn ic3_on_hwmcc20_only_unconstrained_problems() {
        let run_test = false;
        if !run_test {
            return;
        }
        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        for aig_file_path in file_paths {
            if common::_true_with_probability(0.05) {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
                test_ic3::<VarisatSolver>(&fin_state, &aig);
            }
        }
    }

    #[test]
    fn ic3_on_first_few_hwmcc20_unconstrained_problems() {
        let run_test = true;
        if !run_test {
            return;
        }
        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        for aig_file_path in &file_paths[1..5] {
            println!("file_path = {}", aig_file_path);
            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            test_ic3::<SplrSolver>(&fin_state, &aig);
        }
    }
}
