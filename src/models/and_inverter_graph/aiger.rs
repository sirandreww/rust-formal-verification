// ************************************************************************************************
// use
// ************************************************************************************************

use std::fs;

// ************************************************************************************************
// struct
// ************************************************************************************************

// This implementation is in accordance to https://github.com/arminbiere/aiger/blob/master/FORMAT

pub struct AndInverterGraph {}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    fn new() -> Self {
        AndInverterGraph {}
    }

    fn split_vector_of_bytes_to_vector_of_vector_of_bytes_using_newlines(
        vec_of_bytes: &Vec<u8>,
    ) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut current_line: Vec<u8> = Vec::new();
        for byte in vec_of_bytes {
            if byte == &('\n' as u8) {
                result.push(current_line);
                current_line = Vec::new();
            } else {
                current_line.push(byte.to_owned());
            }
        }
        result
    }

    fn check_aig_first_line(
        vector_of_lines_as_vectors: &Vec<Vec<u8>>,
    ) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
        let first_line_as_str = std::str::from_utf8(&vector_of_lines_as_vectors[0]).unwrap();
        let params: Vec<&str> = first_line_as_str.split(' ').collect();

        // check if the input file format is correct (starts with aig)
        assert_eq!(
            params[0], "aig",
            "The parameter line (first line in aig) must start with the word 'aig'."
        );
        assert!(
            params.len() > 5,
            "The parameter line (first line in aig) has too few arguments."
        );

        // first 5 fields always exist
        let maximum_variable_index = params[1].parse::<u32>().unwrap();
        let number_of_inputs = params[2].parse::<u32>().unwrap();
        let number_of_latches = params[3].parse::<u32>().unwrap();
        let number_of_outputs = params[4].parse::<u32>().unwrap();
        let number_of_and_gates = params[5].parse::<u32>().unwrap();

        // these fields do not always exist
        let number_of_bad_outputs = if params.len() > 6 {
            params[6].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_constraint_outputs = if params.len() > 7 {
            params[7].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_justice_outputs = if params.len() > 8 {
            params[8].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_fairness_outputs = if params.len() > 9 {
            params[9].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_outputs = number_of_outputs
            + number_of_bad_outputs
            + number_of_constraint_outputs
            + number_of_justice_outputs
            + number_of_fairness_outputs;

        assert!(
            params.len() < 10,
            "The parameter line (first line in aig) has too many arguments."
        );
        assert_eq!(
            maximum_variable_index,
            number_of_inputs + number_of_latches + number_of_and_gates,
            "The number of variables does not add up."
        );
        assert_eq!(
            number_of_justice_outputs, 0,
            "Reading AIGER files with liveness properties is currently not supported."
        );
        assert_eq!(
            number_of_fairness_outputs, 0,
            "Reading AIGER files with liveness properties is currently not supported."
        );

        if number_of_constraint_outputs > 0 {
            eprintln!("Warning: The last {number_of_constraint_outputs} outputs are interpreted as constraints.");
        }

        return (
            maximum_variable_index,
            number_of_inputs,
            number_of_latches,
            number_of_outputs,
            number_of_and_gates,
            number_of_bad_outputs,
            number_of_constraint_outputs,
            number_of_justice_outputs,
            number_of_fairness_outputs,
        );
    }

    pub fn from_vector_of_bytes(vec_of_bytes: &Vec<u8>) -> AndInverterGraph {
        let lines =
            Self::split_vector_of_bytes_to_vector_of_vector_of_bytes_using_newlines(vec_of_bytes);
        let (nTotal, nInputs, nLatches, nOutputs, nAnds, nBad, nConstr, nJust, nFair) =
            Self::check_aig_first_line(&lines);

        AndInverterGraph::new()
    }

    pub fn from_aig_path(file_path: &str) -> AndInverterGraph {
        let file_as_vec_of_bytes =
            fs::read(file_path).expect(format!("Unable to read the file {file_path}").as_str());
        Self::from_vector_of_bytes(&file_as_vec_of_bytes)
    }
}
