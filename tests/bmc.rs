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

    use crate::common;
    use rust_formal_verification::{
        formulas::literal::VariableType,
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{SatResponse, SplrSolver},
    };
    use std::{
        collections::HashMap,
        time::{Duration, Instant},
    };

    // ********************************************************************************************
    // macro
    // ********************************************************************************************

    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
             let mut map = ::std::collections::HashMap::new();
             $( map.insert($key, $val); )*
             map
        }}
    }

    // ********************************************************************************************
    // Enum
    // ********************************************************************************************

    enum BMCResult {
        NoCTX {
            depth_reached: VariableType,
        },
        CTX {
            assignment: HashMap<VariableType, bool>,
            depth: VariableType,
        },
    }

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn bmc(fsts: &FiniteStateTransitionSystem) -> BMCResult {
        let start = Instant::now();
        for depth in 0.. {
            let elapsed_time = start.elapsed();
            if elapsed_time > Duration::from_secs(5) {
                return BMCResult::NoCTX {
                    depth_reached: depth,
                };
            }

            let mut sat_formula = fsts.get_initial_relation();
            for unroll_depth in 1..(depth + 1) {
                sat_formula.append(&fsts.get_transition_relation_for_some_depth(unroll_depth));
            }
            sat_formula.append(&fsts.get_unsafety_property_for_some_depth(depth));

            let solver = SplrSolver::default();
            let response = solver.solve_cnf(&sat_formula);
            match response {
                SatResponse::Sat { assignment } => {
                    return BMCResult::CTX {
                        assignment: assignment,
                        depth: depth,
                    }
                }
                SatResponse::UnSat => {}
            }
        }
        unreachable!();
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    // *--------------------------------------------------------------*
    // |     _________                                                |
    // |    |         |                                               |
    // *--> | latch 0 | x1-*                                          |
    //      |_________|    |                                          |
    //                     |                                          |
    // *-------------------*-------------Not-------*                  |
    // |     _________                             |     ______       |
    // |    |         |                 ______     *--> |      \      |
    // *--> | latch 1 | x2-*---Not---> |      \         |  and  ) x5--*
    //      |_________|    |           |  and  ) x4---> |______/
    //                     |     *---> |______/
    // *-------------------*     |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn bmc_on_simple_example_counter_with_bad_assertion() {
        let aig =
            AndInverterGraph::from_aig_path("tests/examples/ours/counter_with_bad_assertion.aig");
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        let res = bmc(&fsts);
        match res {
            BMCResult::NoCTX { depth_reached: _ } => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(
                    assignment,
                    hashmap![
                        1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                        6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                        11 =>false, 12 => true, 13 => false, 14 => false, 15 => false,
                        16 => false, 17 => false, 18 => true
                    ]
                );
                assert_eq!(depth, 3);
            }
        }
    }

    // *--------------------------------------------------------------*
    // |     _________                                                |
    // |    |         |                                               |
    // *--> | latch 0 | x1-*                                          |
    //      |_________|    |                                          |
    //                     |                                          |
    // *-------------------*-------------Not-------*                  |
    // |     _________                             |     ______       |
    // |    |         |                 ______     *--> |      \      |
    // *--> | latch 1 | x2--*--Not---> |      \         |  and  ) x5--*
    //      |_________| BAD |           |  and  ) x4---> |______/
    //                      |    *---> |______/
    // *--------------------*    |
    // |     _________          Not
    // |    |         |          |
    // *--> | latch 2 | x3 BAD---*
    //      |_________|
    #[test]
    fn bmc_on_simple_example_counter_with_2_bad_assertion() {
        let aig = AndInverterGraph::from_aig_path(
            "tests/examples/ours/counter_with_2_bad_assertions.aig",
        );
        let fsts = FiniteStateTransitionSystem::from_aig(&aig);

        let res = bmc(&fsts);
        match res {
            BMCResult::NoCTX { depth_reached: _ } => {
                panic!();
            }
            BMCResult::CTX { assignment, depth } => {
                assert_eq!(
                    assignment,
                    hashmap![
                        1 => false, 2 => false, 3 => false, 4 => true, 5 => true,
                        6 => true, 7 => false, 8 => false, 9 => true, 10 => false,
                        11 =>false, 12 => true, 13 => false
                    ]
                );
                assert_eq!(depth, 2);
            }
        }
    }

    #[test]
    fn bmc_on_hwmcc20_only_unconstrained_problems() {
        let file_paths = common::_get_paths_to_hwmcc20_unconstrained();
        for aig_file_path in file_paths {
            println!("{}",aig_file_path);
            let start = Instant::now();
            let aig = AndInverterGraph::from_aig_path(&aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
            let res = bmc(&fin_state);
            match res {
                BMCResult::NoCTX { depth_reached } => {
                    println!(
                        "Seems Ok, ran till depth = {}\t, time = {}",
                        depth_reached, start.elapsed().as_secs_f32()
                    );
                }
                BMCResult::CTX {
                    assignment: _,
                    depth,
                } => {
                    println!(
                        "UNSAFE - CTX found at depth = {}, time = {}",
                        depth, start.elapsed().as_secs_f32()
                    );
                }
            }
        }
    }
}
