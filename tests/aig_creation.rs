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
    use rust_formal_verification::models::AndInverterGraph;
    use std::{cmp::max, fs};

    // ********************************************************************************************
    // helpers
    // ********************************************************************************************

    pub struct AigDetails {
        pub file_name: String,
        pub num_inputs: usize,
        pub num_latches: usize,
        pub num_variables: usize,
        pub num_outputs: usize,
        pub num_bad: usize,
        pub num_constraints: usize,
    }

    fn print_table(table: &Vec<AigDetails>, max_size_of_file_path: usize) {
        println!("AIG files sorted");
        let line = ("AIG file", "input", "latch", "wires", "out", "bad", "const");
        println!(
            "{}{}\t,{}\t,{}\t,{}\t,{}\t,{}\t,{}",
            line.0,
            " ".to_string()
                .repeat(max_size_of_file_path - line.0.chars().count()),
            line.1,
            line.2,
            line.3,
            line.4,
            line.5,
            line.6
        );
        let mut zero_const_files = Vec::new();
        for line in table {
            println!(
                "{}{}\t,{}\t,{}\t,{}\t,{}\t,{}\t,{}",
                line.file_name,
                " ".to_string()
                    .repeat(max_size_of_file_path - line.file_name.chars().count()),
                line.num_inputs,
                line.num_latches,
                line.num_variables,
                line.num_outputs,
                line.num_bad,
                line.num_constraints
            );
            if line.num_constraints == 0 {
                zero_const_files.push(&line.file_name);
            }
        }
        // println!("{:?}", zero_const_files);
    }

    // ********************************************************************************************
    // aig reading test
    // ********************************************************************************************

    #[test]
    fn read_all_aig_files_from_hwmcc20() {
        let file_paths = common::_get_paths_to_all_aig_and_corresponding_aag_files();
        let probability_of_testing_each_file = 0.1;

        let mut table = Vec::new();
        let mut max_size_of_file_path = 0;
        for (aig_file_path, aag_file_path) in file_paths {
            // make the test faster by only doing this with 5% of the files
            if common::_true_with_probability(probability_of_testing_each_file) {
                println!("file_path = {}", aig_file_path);
                max_size_of_file_path = max(max_size_of_file_path, aig_file_path.chars().count());
                let aig = AndInverterGraph::from_aig_path(&aig_file_path);
                table.push(AigDetails {
                    file_name: aig_file_path,
                    num_inputs: aig.get_input_information().len(),
                    num_latches: aig.get_latch_information().len(),
                    num_variables: aig.get_highest_variable_number(),
                    num_outputs: aig.get_output_information().len(),
                    num_bad: aig.get_bad_information().len(),
                    num_constraints: aig.get_constraints_information().len(),
                });
                let aag_received = aig.get_aag_string();
                let true_aag = fs::read_to_string(aag_file_path).unwrap();
                common::_assert_long_string_eq(&true_aag, &aag_received);
            }
        }
        table.sort_by(|a, b| {
            a.num_constraints
                .cmp(&b.num_constraints)
                .then(a.num_variables.cmp(&b.num_variables))
        });
        print_table(&table, max_size_of_file_path);
    }
}
