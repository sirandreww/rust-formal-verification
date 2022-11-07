// ********************************************************************************************
// use
// ********************************************************************************************

use rand::Rng;
use std::cmp::{max, min};
use walkdir::WalkDir;

// ********************************************************************************************
// helper functions to helper functions
// ********************************************************************************************

fn _get_aig_and_aag_files_in_dir(dir: &str) -> Vec<(String, String)> {
    let mut result = Vec::default();
    for dir_entry_result in WalkDir::new(dir) {
        let dir_entry = dir_entry_result.unwrap();
        if dir_entry.path().is_file() {
            let file_path = dir_entry.path().display().to_string();
            if file_path.contains(".aig") {
                // is aig file
                let aag_file_path = file_path.replace(".aig", ".aag");
                result.push((file_path, aag_file_path));
            }
        }
    }
    result.sort();
    result.reverse();
    result
}

// ********************************************************************************************
// helper functions
// ********************************************************************************************

pub fn _assert_long_string_eq(str1: &str, str2: &str) {
    let str1_chars: Vec<char> = str1.chars().collect();
    let str2_chars: Vec<char> = str2.chars().collect();
    if str1_chars == str2_chars {
        // they are the same do nothing
        assert_eq!(str1, str2);
    } else {
        // find the first location they differ
        let mut first_index_they_differ = usize::MAX;

        let for_length = min(str1_chars.len(), str2_chars.len());
        for i in 0..for_length {
            if str1_chars[i] != str2_chars[i] {
                first_index_they_differ = i;
                break;
            }
        }

        if first_index_they_differ == usize::MAX {
            // they are appear to be the same, but must have different sizes
            first_index_they_differ = for_length;
        }

        let start = max(first_index_they_differ - 20, 0);

        let end1 = min(first_index_they_differ + 80, str1_chars.len());
        let str1_short: String = str1_chars[start..end1].iter().collect();

        let end2 = min(first_index_they_differ + 80, str2_chars.len());
        let str2_short: String = str2_chars[start..end2].iter().collect();

        assert_eq!(str1_short, str2_short);
        // just in case runtime gets here
        assert_eq!(str1, str2);
    }
}

pub fn _get_paths_to_all_aig_and_corresponding_aag_files() -> Vec<(String, String)> {
    _get_aig_and_aag_files_in_dir("tests/examples/hwmcc20")
}

pub fn _true_with_probability(prob: f64) -> bool {
    assert!(0.0 <= prob && prob <= 1.0);
    let mut rng = rand::thread_rng();
    let random_number_between_0_and_1: f64 = rng.gen();
    return random_number_between_0_and_1 > (1.0 - prob);
}

// pub fn _get_paths_to_all_aig_for_2020() -> Vec<String> {
//     let mut result = Vec::default();
//     for aig_file_result in WalkDir::new("tests/hwmcc20_aig/2020") {
//         let aig_file = aig_file_result.unwrap();
//         if aig_file.path().is_file() {
//             let aig_file_path = aig_file.path().display().to_string();
//             result.push(aig_file_path);
//         }
//     }
//     result.sort();
//     result
// }

pub fn _get_paths_to_all_our_example_aig_files() -> Vec<String> {
    _get_aig_and_aag_files_in_dir("tests/examples/ours")
        .iter()
        .map(|t| t.to_owned().0)
        .collect()
}

pub fn _get_paths_to_hwmcc20_unconstrained() -> Vec<String> {
    let result = vec!["tests/examples/hwmcc20/2019/goel/crafted/paper_v3/paper_v3.aig", "tests/examples/hwmcc20/2019/goel/opensource/vcegar_QF_BV_itc99_b13_p10/vcegar_QF_BV_itc99_b13_p10.aig", "tests/examples/hwmcc20/2020/mann/simple_alu.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_bufferAlloc/vis_arrays_bufferAlloc.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_buf_bug/vis_arrays_buf_bug.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p2/vis_arrays_am2910_p2.aig", "tests/examples/hwmcc20/2019/goel/opensource/miim/miim.aig", "tests/examples/hwmcc20/2019/goel/industry/cal21/cal21.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2901/vis_arrays_am2901.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p1/vis_arrays_am2910_p1.aig", "tests/examples/hwmcc20/2019/goel/opensource/vis_arrays_am2910_p3/vis_arrays_am2910_p3.aig", "tests/examples/hwmcc20/2019/goel/opensource/h_TreeArb/h_TreeArb.aig", "tests/examples/hwmcc20/2019/beem/krebs.3.prop1-func-interl.aig", "tests/examples/hwmcc20/2019/goel/industry/cal41/cal41.aig", "tests/examples/hwmcc20/2019/beem/mcs.3.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/industry/cal4/cal4.aig", "tests/examples/hwmcc20/2019/beem/brp2.2.prop1-func-interl.aig", "tests/examples/hwmcc20/2019/beem/elevator.4.prop1-func-interl.aig", "tests/examples/hwmcc20/2019/beem/at.6.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/opensource/h_RCU/h_RCU.aig", "tests/examples/hwmcc20/2019/beem/anderson.3.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/industry/cal35/cal35.aig", "tests/examples/hwmcc20/2019/goel/industry/cal37/cal37.aig", "tests/examples/hwmcc20/2019/beem/brp2.3.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/beem/brp2.6.prop3-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/opensource/vcegar_arrays_itc99_b12_p2/vcegar_arrays_itc99_b12_p2.aig", "tests/examples/hwmcc20/2019/beem/msmie.3.prop1-func-interl.aig", "tests/examples/hwmcc20/2019/goel/industry/cal33/cal33.aig", "tests/examples/hwmcc20/2019/beem/blocks.4.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/industry/gen44/gen44.aig", "tests/examples/hwmcc20/2019/goel/industry/gen43/gen43.aig", "tests/examples/hwmcc20/2019/goel/industry/gen21/gen21.aig", "tests/examples/hwmcc20/2019/goel/industry/gen10/gen10.aig", "tests/examples/hwmcc20/2019/goel/industry/gen14/gen14.aig", "tests/examples/hwmcc20/2019/goel/industry/gen12/gen12.aig", "tests/examples/hwmcc20/2019/goel/industry/gen39/gen39.aig", "tests/examples/hwmcc20/2019/goel/industry/cal34/cal34.aig", "tests/examples/hwmcc20/2019/goel/industry/gen31/gen31.aig", "tests/examples/hwmcc20/2019/beem/frogs.5.prop1-func-interl.aig", "tests/examples/hwmcc20/2019/goel/industry/gen35/gen35.aig", "tests/examples/hwmcc20/2019/goel/industry/mul1/mul1.aig", "tests/examples/hwmcc20/2019/goel/industry/mul9/mul9.aig", "tests/examples/hwmcc20/2019/beem/pgm_protocol.3.prop5-func-interl.aig", "tests/examples/hwmcc20/2019/beem/peg_solitaire.3.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/industry/cal84/cal84.aig", "tests/examples/hwmcc20/2019/goel/industry/cal2/cal2.aig", "tests/examples/hwmcc20/2019/wolf/2018D/picorv32-pcregs-p2.aig", "tests/examples/hwmcc20/2019/wolf/2018D/picorv32-pcregs-p0.aig", "tests/examples/hwmcc20/2019/wolf/2018D/ponylink-slaveTXlen-unsat.aig", "tests/examples/hwmcc20/2019/goel/industry/cal87/cal87.aig", "tests/examples/hwmcc20/2019/goel/industry/cal90/cal90.aig", "tests/examples/hwmcc20/2019/goel/industry/cal86/cal86.aig", "tests/examples/hwmcc20/2019/goel/industry/cal149/cal149.aig", "tests/examples/hwmcc20/2019/goel/industry/cal142/cal142.aig", "tests/examples/hwmcc20/2019/goel/industry/cal118/cal118.aig", "tests/examples/hwmcc20/2019/goel/industry/cal140/cal140.aig", "tests/examples/hwmcc20/2019/goel/industry/cal117/cal117.aig", "tests/examples/hwmcc20/2019/goel/industry/cal99/cal99.aig", "tests/examples/hwmcc20/2019/goel/industry/cal129/cal129.aig", "tests/examples/hwmcc20/2019/goel/industry/cal123/cal123.aig", "tests/examples/hwmcc20/2019/goel/industry/cal122/cal122.aig", "tests/examples/hwmcc20/2019/goel/industry/cal143/cal143.aig", "tests/examples/hwmcc20/2019/goel/industry/cal125/cal125.aig", "tests/examples/hwmcc20/2019/goel/industry/cal97/cal97.aig", "tests/examples/hwmcc20/2019/goel/industry/cal119/cal119.aig", "tests/examples/hwmcc20/2019/goel/industry/cal106/cal106.aig", "tests/examples/hwmcc20/2019/goel/industry/cal107/cal107.aig", "tests/examples/hwmcc20/2019/goel/industry/cal112/cal112.aig", "tests/examples/hwmcc20/2019/goel/industry/cal102/cal102.aig", "tests/examples/hwmcc20/2019/beem/pgm_protocol.7.prop1-back-serstep.aig", "tests/examples/hwmcc20/2019/goel/opensource/vcegar_QF_BV_ar/vcegar_QF_BV_ar.aig", "tests/examples/hwmcc20/2020/mann/stack-p2.aig", "tests/examples/hwmcc20/2020/mann/stack-p1.aig", "tests/examples/hwmcc20/2019/goel/industry/cal159/cal159.aig", "tests/examples/hwmcc20/2019/goel/industry/cal162/cal162.aig", "tests/examples/hwmcc20/2019/goel/industry/cal161/cal161.aig", "tests/examples/hwmcc20/2019/beem/rushhour.4.prop1-func-interl.aig", "tests/examples/hwmcc20/2020/mann/rast-p21.aig", "tests/examples/hwmcc20/2020/mann/rast-p19.aig", "tests/examples/hwmcc20/2020/mann/rast-p18.aig", "tests/examples/hwmcc20/2020/mann/rast-p17.aig", "tests/examples/hwmcc20/2020/mann/rast-p16.aig", "tests/examples/hwmcc20/2020/mann/rast-p14.aig", "tests/examples/hwmcc20/2020/mann/rast-p11.aig", "tests/examples/hwmcc20/2020/mann/rast-p06.aig", "tests/examples/hwmcc20/2020/mann/rast-p04.aig", "tests/examples/hwmcc20/2020/mann/rast-p03.aig", "tests/examples/hwmcc20/2020/mann/rast-p01.aig", "tests/examples/hwmcc20/2020/mann/rast-p00.aig", "tests/examples/hwmcc20/2019/goel/industry/mul2/mul2.aig", "tests/examples/hwmcc20/2019/goel/industry/cal81/cal81.aig", "tests/examples/hwmcc20/2019/goel/industry/cal209/cal209.aig", "tests/examples/hwmcc20/2019/goel/industry/cal156/cal156.aig", "tests/examples/hwmcc20/2019/goel/industry/cal210/cal210.aig", "tests/examples/hwmcc20/2019/goel/industry/cal192/cal192.aig", "tests/examples/hwmcc20/2019/goel/industry/cal206/cal206.aig", "tests/examples/hwmcc20/2019/goel/industry/cal201/cal201.aig", "tests/examples/hwmcc20/2019/goel/industry/cal176/cal176.aig", "tests/examples/hwmcc20/2019/goel/industry/cal227/cal227.aig", "tests/examples/hwmcc20/2019/goel/industry/cal220/cal220.aig", "tests/examples/hwmcc20/2019/goel/industry/cal234/cal234.aig", "tests/examples/hwmcc20/2019/goel/industry/cal224/cal224.aig", "tests/examples/hwmcc20/2019/goel/industry/mul3/mul3.aig", "tests/examples/hwmcc20/2019/goel/industry/mul7/mul7.aig"];
    assert_eq!(result.len(), 104); // However, not all bit-blasted AIGER models had constraints (actually 104 did not).
    result.iter().map(|s| s.to_string()).collect()
}
