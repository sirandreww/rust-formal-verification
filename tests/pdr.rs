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
        algorithms::{pdr::PDRResult, PDR},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{CadicalSolver, SatSolver, SplrSolver, VarisatSolver},
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_pdr<T: SatSolver>(fin_state: &FiniteStateTransitionSystem, _aig: &AndInverterGraph) {
        let mut pdr_solver = PDR::<T>::new(fin_state, true);
        let start_time = time::Instant::now();
        let prove_result = pdr_solver.prove();
        let duration = start_time.elapsed();
        println!("Elapsed time = {}", duration.as_secs_f32());
        match prove_result {
            PDRResult::Proof { invariant } => {
                println!("Safe, checking invariant.");
                fin_state.check_invariant::<T>(&invariant);
                println!("Invariant check passed!");
            }
            PDRResult::CTX { depth } => {
                // do nothing for now
                println!("Unsafe, depth = {}", depth);
            }
        }
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn pdr_on_our_examples() {
        let file_paths = common::_get_paths_to_all_our_example_aig_files();
        for aig_file_path in file_paths {
            println!("file_path = {}", aig_file_path);

            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, false);
            test_pdr::<SplrSolver>(&fin_state, &aig);
            test_pdr::<VarisatSolver>(&fin_state, &aig);
            test_pdr::<CadicalSolver>(&fin_state, &aig);
        }
    }

    #[test]
    fn pdr_on_hwmcc20_only_unconstrained_problems() {
        let run_test = false;
        if !run_test {
            return;
        }
        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        for aig_file_path in file_paths {
            if common::_true_with_probability(0.05) {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let fin_state = FiniteStateTransitionSystem::from_aig(&aig, false);
                test_pdr::<VarisatSolver>(&fin_state, &aig);
            }
        }
    }

    #[test]
    fn pdr_on_first_few_hwmcc20_unconstrained_problems() {
        let run_test = false;
        if !run_test {
            return;
        }

        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        let easy_problems = vec![1, 2, 5, 8, 9, 10, 13];

        for (i, aig_file_path) in file_paths
            .iter()
            .enumerate()
            .filter(|(i, _)| easy_problems.contains(i))
        {
            println!("i = {}, file_path = {}", i, aig_file_path);
            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, false);
            // test_ic3::<SplrSolver>(&fin_state, &aig);
            test_pdr::<VarisatSolver>(&fin_state, &aig);
            // test_ic3::<CadicalSolver>(&fin_state, &aig);
        }
    }
}
