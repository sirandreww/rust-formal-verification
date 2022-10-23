// ********************************************************************************************
// use
// ********************************************************************************************

use std::cmp::{max, min};
use walkdir::WalkDir;

// ********************************************************************************************
// helper functions
// ********************************************************************************************

pub fn assert_long_string_eq(str1: &str, str2: &str) {
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

pub fn get_paths_to_all_aig_and_corresponding_aag_files() -> Vec<(String, String)> {
    let mut result = Vec::default();
    for aig_file_result in WalkDir::new("tests/hwmcc20_aig") {
        let aig_file = aig_file_result.unwrap();
        if aig_file.path().is_file() {
            let aig_file_path = aig_file.path().display().to_string();

            let mut file_path_parsed = aig_file_path.split('/').collect::<Vec<&str>>();
            let file_name = file_path_parsed[file_path_parsed.len() - 1].replace(".aig", ".aag");
            let len = file_path_parsed.len();
            file_path_parsed[1] = "hwmcc20_aag";
            file_path_parsed[len - 1] = &file_name;
            let aag_file_path = file_path_parsed.join("/");

            result.push((aig_file_path, aag_file_path));
        }
    }
    result.sort();
    result.reverse();
    result
}
