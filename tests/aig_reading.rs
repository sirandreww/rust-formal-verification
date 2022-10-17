#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::models::AndInverterGraph;
    use std::{fs};
    use walkdir::WalkDir;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn read_aig(file_path: &str) {
        let aig = AndInverterGraph::from_aig_path(file_path);
        let aag_received = aig.get_aag_string();

        let mut file_path_parsed = file_path.split("/").collect::<Vec<&str>>();
        let file_name = file_path_parsed[file_path_parsed.len() -1].replace(".aig", ".aag");
        let len = file_path_parsed.len();

        file_path_parsed[1] = "hwmcc20_aag";
        file_path_parsed[len -1] = file_name.as_str();

        let aag_path = file_path_parsed.join("/");
        let true_aag = fs::read_to_string(aag_path).unwrap();

        assert!(true_aag.contains(&aag_received));
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
            read_aig(aig_file_path.as_str());
        }
    }
}
