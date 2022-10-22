#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::models::AndInverterGraph;
    use std::{
        cmp::{max, min},
        fs,
    };
    use walkdir::WalkDir;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn assert_long_string_eq(str1: &str, str2: &str) {
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

            let start = max((first_index_they_differ - 20).into(), 0);

            let end1 = min(first_index_they_differ + 80, str1_chars.len());
            let str1_short: String = str1_chars[start..end1].into_iter().collect();

            let end2 = min(first_index_they_differ + 80, str2_chars.len());
            let str2_short: String = str2_chars[start..end2].into_iter().collect();

            assert_eq!(str1_short, str2_short);
            // just in case runtime gets here
            assert_eq!(str1, str2);
        }
    }

    fn read_aig(file_path: &str) {
        let aig = AndInverterGraph::from_aig_path(file_path);
        let aag_received = aig.get_aag_string();

        let mut file_path_parsed = file_path.split('/').collect::<Vec<&str>>();
        let file_name = file_path_parsed[file_path_parsed.len() - 1].replace(".aig", ".aag");
        let len = file_path_parsed.len();

        file_path_parsed[1] = "hwmcc20_aag";
        file_path_parsed[len - 1] = &file_name;

        let aag_path = file_path_parsed.join("/");
        let true_aag = fs::read_to_string(aag_path).unwrap();

        assert_long_string_eq(&true_aag, &aag_received);
    }

    fn get_paths_to_all_aig_files() -> Vec<String> {
        let mut result = Vec::default();
        for aig_file_result in WalkDir::new("tests/hwmcc20_aig") {
            let aig_file = aig_file_result.unwrap();
            if aig_file.path().is_file() {
                let file_path = aig_file.path().display().to_string();
                result.push(file_path);
            }
        }
        result.sort();
        result.reverse();
        result
    }

    // ********************************************************************************************
    // aig reading test
    // ********************************************************************************************

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let aig_file_paths = get_paths_to_all_aig_files();
        for aig_file_path in aig_file_paths {
            println!("file_path = {}", aig_file_path);
            read_aig(&aig_file_path);
        }
    }
}
