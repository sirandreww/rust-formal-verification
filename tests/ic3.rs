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
        algorithms::{ic3::IC3Result, IC3},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{
            stateless::{CadicalSolver, SplrSolver, VarisatSolver},
            StatelessSatSolver,
        },
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_ic3<T: StatelessSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
        _aig: &AndInverterGraph,
    ) {
        let mut ic3_solver = IC3::<T>::new(fin_state, true);
        let start_time = time::Instant::now();
        let prove_result = ic3_solver.prove();
        let duration = start_time.elapsed();
        println!("Elapsed time = {}", duration.as_secs_f32());
        match prove_result {
            IC3Result::Proof { invariant } => {
                println!("Safe, checking invariant.");
                fin_state.check_invariant::<T>(&invariant);
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
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, false);
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
                let fin_state = FiniteStateTransitionSystem::from_aig(&aig, false);
                test_ic3::<VarisatSolver>(&fin_state, &aig);
            }
        }
    }

    #[test]
    fn ic3_on_few_hwmcc20_folded_problems() {
        let run_test = false;
        if !run_test {
            return;
        }

        let file_paths = vec![
            "tests/examples/hwmcc20/2019/beem/brp2.3.prop1-back-serstep_zero_then_fold2.aig",
            // "tests/examples/hwmcc20/2019/mann/data-integrity/unsafe/circular_pointer_top_w8_d16_e0_zero_then_fold2.aig"
        ];

        for (i, aig_file_path) in file_paths.iter().enumerate() {
            println!(
                "i = {}/{}, file_path = {}",
                i,
                file_paths.len(),
                aig_file_path
            );
            let aig = AndInverterGraph::from_aig_path(aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, true);
            // test_ic3::<SplrSolver>(&fin_state, &aig);
            test_ic3::<VarisatSolver>(&fin_state, &aig);
            // test_ic3::<CadicalSolver>(&fin_state, &aig);
        }
    }
}
