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
        algorithms::{ic3::IC3Result, IC3},
        formulas::CNF,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn check_invariant(_fin_state: &FiniteStateTransitionSystem, _inv_candidate: &CNF) {
        // check INIT -> inv_candidate
        // let init = fin_state.get_initial_relation();
    }

    fn test_ic3(fin_state: &FiniteStateTransitionSystem, _aig: &AndInverterGraph) {
        let mut ic3_solver = IC3::new(fin_state);
        match ic3_solver.prove() {
            IC3Result::Proof { invariant } => {
                println!("Safe, checking invariant.");
                check_invariant(fin_state, &invariant);
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

    // #[test]
    // fn pdr_on_2020_examples() {
    //     let file_paths = common::_get_paths_to_all_aig_for_2020();
    //     for aig_file_path in file_paths {
    //         println!("file_path = {}", aig_file_path);

    //         let aig = AndInverterGraph::from_aig_path(&aig_file_path);
    //         let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
    //         ic3(&fin_state, &aig);
    //     }
    // }

    #[test]
    fn ic3_on_our_examples() {
        let file_paths = common::_get_paths_to_all_our_example_aig_files();
        for aig_file_path in file_paths {
            println!("file_path = {}", aig_file_path);

            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            test_ic3(&fin_state, &aig);
        }
    }

    #[test]
    fn ic3_on_hwmcc20_only_unconstrained_problems() {
        let run_test = true;
        if run_test {
            let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
            for aig_file_path in file_paths {
                if common::_true_with_probability(0.05) {
                    println!("file_path = {}", aig_file_path);
                    let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                    let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
                    test_ic3(&fin_state, &aig);
                }
            }
        }
    }
}
