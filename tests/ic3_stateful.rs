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
        algorithms::proof::{IC3Stateful, ProofResult, RFV1},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{
            stateful::{CaDiCalSolver as StateFulCaDiCal, StatefulSatSolver},
            stateless::CaDiCalSolver,
        },
    };

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn average(numbers: &[f32]) -> f32 {
        numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
    }

    fn call_first_prover<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
    ) -> (ProofResult, Duration) {
        let mut ic3_solver = IC3Stateful::<T>::new(fin_state, true);
        let start_time = time::Instant::now();
        let prove_result = ic3_solver.prove();
        let duration = start_time.elapsed();
        (prove_result, duration)
    }

    fn call_second_prover<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
    ) -> (ProofResult, Duration) {
        let mut ic3_solver = RFV1::<T>::new(fin_state, true);
        let start_time = time::Instant::now();
        let prove_result = ic3_solver.prove();
        let duration = start_time.elapsed();
        (prove_result, duration)
    }

    fn test<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
        is_first: bool,
        _aig: &AndInverterGraph,
    ) -> Duration {
        let (prove_result, duration) = if is_first {
            call_first_prover::<T>(fin_state)
        } else {
            call_second_prover::<T>(fin_state)
        };

        println!("Elapsed time = {}", duration.as_secs_f32());
        match prove_result {
            ProofResult::Proof { invariant } => {
                println!("Safe, checking invariant.");
                fin_state.check_invariant::<CaDiCalSolver>(&invariant);
                println!("Invariant check passed!");
            }
            ProofResult::CTX { depth } => {
                // do nothing for now
                println!("Unsafe, depth = {}", depth);
            }
        }
        duration
    }

    // ********************************************************************************************
    // tests
    // ********************************************************************************************

    #[test]
    fn ic3_stateful_on_few_hwmcc20_folded_problems() {
        let run_test = true;
        if !run_test {
            return;
        }

        let file_paths = vec![
            // 0 to 2 seconds
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divthree-p143_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p2/vis_arrays_am2910_p2_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/zipversa_composecrc_prf-p11_zero_then_fold2.aig",
// "tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divfive-p143_zero_then_fold2.aig",
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

            // 2 seconds to 25
"tests/examples/hwmcc20/2019/wolf/2019B/marlann_compute_cp_pass-p2_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/opensource/h_TreeArb/h_TreeArb_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019B/marlann_compute_cp_fail1-p2_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/opensource/miim/miim_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/industry/gen39/gen39_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p122_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/industry/cal35/cal35_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2018D/zipcpu-busdelay-p43_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/beem/at.6.prop1-back-serstep_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/industry/cal33/cal33_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/industry/cal37/cal37_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p048_zero_then_fold2.aig",
"tests/examples/hwmcc20/2020/mann/rast-p11_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_qflexpress_divfive-p038_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/beem/brp2.3.prop1-back-serstep_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/mann/data-integrity/unsafe/shift_register_top_w16_d8_e0_zero_then_fold2.aig",
        ];

        let mut time1 = Vec::new();
        let mut time2 = Vec::new();

        for (i, aig_file_path) in file_paths.iter().enumerate() {
            println!(
                "i = {}/{}, file_path = {}",
                i,
                file_paths.len(),
                aig_file_path
            );
            let aig = AndInverterGraph::from_aig_path(aig_file_path);
            let fin_state = FiniteStateTransitionSystem::from_aig(&aig, true);

            let t1 = test::<StateFulCaDiCal>(&fin_state, true, &aig).as_secs_f32();
            let t2 = test::<StateFulCaDiCal>(&fin_state, false, &aig).as_secs_f32();

            time1.push(t1);
            time2.push(t2);
            println!("************************************************************************");
            println!("Average time for first prover = {}", average(&time1));
            println!("Average time for second prover = {}", average(&time2));
        }
    }
}
