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

    use crate::common;
    use rand::Rng;
    use rust_formal_verification::models::AndInverterGraph;
    use std::fs;

    // ********************************************************************************************
    // aig reading test
    // ********************************************************************************************

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let file_paths = common::_get_paths_to_all_aig_and_corresponding_aag_files();
        for (aig_file_path, aag_file_path) in file_paths {
            let mut rng = rand::thread_rng();
            let random_number_between_0_and_1: f64 = rng.gen();
            // make the test faster by only doing this with 5% of the files
            if random_number_between_0_and_1 > 0.95 {
                println!("file_path = {}", aig_file_path);
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                let aag_received = aig.get_aag_string();
                let true_aag = fs::read_to_string(aag_file_path).unwrap();
                common::_assert_long_string_eq(&true_aag, &aag_received);
            }
        }
    }
}
