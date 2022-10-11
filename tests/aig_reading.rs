#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::models::AndInverterGraph;
    use walkdir::WalkDir;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn read_aig(file_path: &str) {
        let _aig = AndInverterGraph::from_aig_path(file_path);
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
    fn try_some_aig() {
        let aig_file_paths = get_paths_to_all_aig_files();
        for aig_file_path in aig_file_paths {
            println!("file_path = {}", aig_file_path);
            read_aig(aig_file_path.as_str());
        }
    }
}
