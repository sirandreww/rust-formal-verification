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

    use std::time::{self, Duration};

    use rust_formal_verification::{
        algorithms::proof::{IC3Stateful, ProofResult, PDR},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{
            stateful::{CaDiCalSolver as StateFulCaDiCal, StatefulSatSolver},
            stateless::CaDiCalSolver,
        },
    };

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn average(numbers: &[f32]) -> f32 {
        numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
    }

    fn call_first_prover<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
    ) -> ProofResult {
        let mut prover = PDR::<T>::new(fin_state, true);
        prover.prove()
    }

    fn call_second_prover<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
    ) -> ProofResult {
        let mut prover = IC3Stateful::<T>::new(fin_state, false);
        prover.prove()
    }

    fn test<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
        is_first: bool,
    ) -> (ProofResult, Duration) {
        let start_time = time::Instant::now();
        let prove_result = if is_first {
            call_first_prover::<T>(fin_state)
        } else {
            call_second_prover::<T>(fin_state)
        };
        let duration = start_time.elapsed();

        println!("Elapsed time = {}", duration.as_secs_f32());
        match prove_result {
            ProofResult::Proof { invariant } => {
                if common::_true_with_probability(if is_first { 1.0 } else { 0.05 }) {
                    println!("Safe, checking invariant.");
                    fin_state.check_invariant::<CaDiCalSolver>(&invariant);
                    println!("Invariant check passed!");
                }
                (ProofResult::Proof { invariant }, duration)
            }
            ProofResult::CTX { depth } => {
                // do nothing for now
                println!("Unsafe, depth = {}", depth);
                (prove_result, duration)
            }
        }
    }

    fn call_test_and_make_sure_same_result<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
    ) -> (Duration, Duration) {
        let (r1, t1) = test::<T>(&fin_state, true);
        let (r2, t2) = test::<T>(&fin_state, false);
        match (r1, r2) {
            (ProofResult::Proof { invariant: _ }, ProofResult::Proof { invariant: _ }) => {}
            (ProofResult::CTX { depth: _ }, ProofResult::CTX { depth: _ }) => {}
            _ => {
                panic!("Provers disagree");
            }
        };
        (t1, t2)
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn compare_2_provers() {
        let run_test = true;
        if !run_test {
            return;
        }

        let file_paths = vec![
            // 0 to 2 seconds
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divthree-p143_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p2/vis_arrays_am2910_p2_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/zipversa_composecrc_prf-p11_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divfive-p143_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/zipversa_composecrc_prf-p07_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p1/vis_arrays_am2910_p1_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/simple_alu_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/vcegar_QF_BV_itc99_b13_p10/vcegar_QF_BV_itc99_b13_p10_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/stack-p1_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/beem/anderson.3.prop1-back-serstep_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2901/vis_arrays_am2901_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/zipversa_composecrc_prf-p00_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p04_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p01_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p03_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p06_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p19_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p18_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p21_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p3/vis_arrays_am2910_p3_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/gen21/gen21_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal41/cal41_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/mann/safe/intersymbol_analog_estimation_convergence_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divfive-p022_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divthree-p158_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal21/cal21_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/gen14/gen14_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/gen10/gen10_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/gen12/gen12_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divfive-p016_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal4/cal4_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p017_zero_then_fold2.aig",

//             // 2 seconds to 25
// "tests/examples/hwmcc20/2019/wolf/2019B/marlann_compute_cp_pass-p2_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/h_TreeArb/h_TreeArb_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019B/marlann_compute_cp_fail1-p2_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/miim/miim_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/gen39/gen39_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p122_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal35/cal35_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2018D/zipcpu-busdelay-p43_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/beem/at.6.prop1-back-serstep_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal33/cal33_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/industry/cal37/cal37_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p048_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2020/mann/rast-p11_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p038_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/beem/brp2.3.prop1-back-serstep_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/mann/data-integrity/unsafe/shift_register_top_w16_d8_e0_zero_then_fold2.aig",
        ];

        let mut time1 = Vec::new();
        let mut time2 = Vec::new();

        for (i, aig_file_path) in file_paths.iter().enumerate() {
            println!(
                "i = {}/{}, file_path = {}",
                i + 1,
                file_paths.len(),
                aig_file_path
            );
            let aig = AndInverterGraph::from_aig_path(aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, true);
            let mut t1_for_test_i = Vec::new();
            let mut t2_for_test_i = Vec::new();
            for _ in 0..10 {
                let (t1, t2) = call_test_and_make_sure_same_result::<StateFulCaDiCal>(&fin_state);
                t1_for_test_i.push(t1.as_secs_f32());
                t2_for_test_i.push(t2.as_secs_f32());
                // println!("Current average time for first prover on this test  = {}", average(&t1_for_test_i));
                // println!("Current average time for second prover on this test = {}", average(&t2_for_test_i));
            }
            time1.push(average(&t1_for_test_i));
            time2.push(average(&t2_for_test_i));
            println!("************************************************************************");
            println!("Time for first prover  = {:?}", time1);
            println!("Time for second prover = {:?}", time2);
            println!("Average time for first prover  = {}", average(&time1));
            println!("Average time for second prover = {}", average(&time2));
            println!("************************************************************************");
        }
    }
}
