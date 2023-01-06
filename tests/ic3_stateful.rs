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
        algorithms::proof::{IC3Stateful, ProofResult},
        models::{AndInverterGraph, FiniteStateTransitionSystem},
        solvers::sat::{
            stateful::{CaDiCalSolver as StateFulCaDiCal, MiniSatSolver, StatefulSatSolver},
            stateless::CaDiCalSolver,
        },
    };

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn average(numbers: &[f32]) -> f32 {
        numbers.iter().sum::<f32>() as f32 / numbers.len() as f32
    }

    fn test<T: StatefulSatSolver>(
        fin_state: &FiniteStateTransitionSystem,
        _aig: &AndInverterGraph,
    ) -> Duration {
        let mut ic3_solver = IC3Stateful::<T>::new(fin_state, true);
        let start_time = time::Instant::now();
        let prove_result = ic3_solver.prove();
        let duration = start_time.elapsed();
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
"tests/examples/hwmcc20/2019/wolf/2019C/qspiflash_dualflexpress_divthree-p143_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p2/vis_arrays_am2910_p2_zero_then_fold2.aig",
"tests/examples/hwmcc20/2019/wolf/2019C/zipversa_composecrc_prf-p11_zero_then_fold2.aig",
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
            time1.push(test::<MiniSatSolver>(&fin_state, &aig).as_secs_f32());
            time2.push(test::<StateFulCaDiCal>(&fin_state, &aig).as_secs_f32());
        }
        println!("Average time for first prover = {}", average(&time1));
        println!("Average time for second prover = {}", average(&time2));
    }
}
