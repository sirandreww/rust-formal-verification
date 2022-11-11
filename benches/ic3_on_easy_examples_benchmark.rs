// ************************************************************************************************
// use declaration
// ************************************************************************************************

use ::criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_formal_verification::{
    algorithms::IC3,
    models::{AndInverterGraph, FiniteStateTransitionSystem},
    solvers::sat::VarisatSolver,
};

// ************************************************************************************************
// helper functions
// ************************************************************************************************

// ************************************************************************************************
// benchmark
// ************************************************************************************************

fn ic3_on_first_few_hwmcc20_unconstrained_problems(c: &mut Criterion) {
    let file_paths = black_box(vec![
        "tests/examples/hwmcc20/2019/goel/opensource/vcegar_QF_BV_itc99_b13_p10/vcegar_QF_BV_itc99_b13_p10.aig", 
        "tests/examples/hwmcc20/2020/mann/simple_alu.aig",
        "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p2/vis_arrays_am2910_p2.aig",
        // "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2901/vis_arrays_am2901.aig"
        "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p1/vis_arrays_am2910_p1.aig",
        ]
    );

    c.bench_function("ic3 on short tests", |b| {
        b.iter(|| {
            for aig_file_path in &file_paths {
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let fin_state = FiniteStateTransitionSystem::from_aig(&aig);
                let mut ic3_solver = IC3::<VarisatSolver>::new(&fin_state, false);
                ic3_solver.prove();
            }
        })
    });
}

// ************************************************************************************************
// link benchmark
// ************************************************************************************************

criterion_group!(benches, ic3_on_first_few_hwmcc20_unconstrained_problems);
criterion_main!(benches);
