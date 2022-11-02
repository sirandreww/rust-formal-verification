// ********************************************************************************************
// use
// ********************************************************************************************

use std::cmp::{max, min};
use walkdir::WalkDir;
use rand::Rng;

// ********************************************************************************************
// helper functions to helper functions
// ********************************************************************************************

fn get_aig_and_aag_files_in_dir(dir: &str) -> Vec<(String, String)> {
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
    get_aig_and_aag_files_in_dir("tests/examples/hwmcc20")
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
    get_aig_and_aag_files_in_dir("tests/examples/ours")
        .iter()
        .map(|t| t.to_owned().0)
        .collect()
}
