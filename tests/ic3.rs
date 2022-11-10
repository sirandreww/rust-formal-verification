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

    use std::time;

    use rust_formal_verification::{
        algorithms::{ic3::IC3Result, IC3, formula_logic::{check_invariant}},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{CadicalSolver, SatSolver, SplrSolver, VarisatSolver},
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_ic3<T: SatSolver>(fin_state: &FiniteStateTransitionSystem, _aig: &AndInverterGraph) {
        let mut ic3_solver = IC3::<T>::new(fin_state);
        let start_time = time::Instant::now();
        let prove_result = ic3_solver.prove();
        let duration = start_time.elapsed();
        println!("Elapsed time = {}", duration.as_secs_f32());
        match prove_result {
            IC3Result::Proof { invariant } => {
                println!("Safe, checking invariant.");
                check_invariant::<T>(fin_state, &invariant);
                println!("Invariant check passed!");
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
        let number_of_tests_to_do = 5;

        for aig_file_path in &file_paths[0..number_of_tests_to_do] {
            println!("file_path = {}", aig_file_path);
            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            // test_ic3::<SplrSolver>(&fin_state, &aig);
            test_ic3::<VarisatSolver>(&fin_state, &aig);
            // test_ic3::<CadicalSolver>(&fin_state, &aig);
        }
    }
}
