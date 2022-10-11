// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::AIGNode;
use crate::models::and_inverter_graph::aig_node::AIGNodeType;
use std::fs;

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct AndInverterGraph {
    maximum_variable_index:             u32,
    number_of_inputs:                   u32,
    number_of_latches:                  u32,
    number_of_outputs:                  u32,
    number_of_and_gates:                u32,
    number_of_bad_state_constraints:    u32,
    number_of_invariant_constraints:    u32,
    number_of_justice_constraints:      u32,
    number_of_fairness_constraints:     u32,

    aig_nodes: Vec<AIGNode>,

    // indexes_of_input_nodes:             Vec<u32>,
    // indexes_of_latch_nodes:             Vec<u32>,
    // indexes_of_and_node:                Vec<u32>,
    // indexes_of_output_literals:         Vec<u32>,
    // indexes_of_bad_literals:            Vec<u32>,
    // indexes_of_constraints_literals:    Vec<u32>,
    // indexes_of_justice_literals:        Vec<u32>,
    // indexes_of_fairness_literals:       Vec<u32>,
    vPis: Vec<usize>,          // the array of primary inputs
    vPos: Vec<usize>,          // the array of primary outputs
    vCis: Vec<usize>,          // the array of combinational inputs  (PIs, latches)
    vCos: Vec<usize>,          // the array of combinational outputs (POs, asserts, latches)
    vPios: Vec<usize>,         // the array of PIOs
    vBoxes: Vec<usize>,        // the array of boxes

}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn split_vector_by_newline(vec_of_bytes: &[u8]) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut current_line: Vec<u8> = Vec::new();
        for byte in vec_of_bytes {
            if byte == &b'\n' {
                result.push(current_line);
                current_line = Vec::new();
            } else {
                current_line.push(byte.to_owned());
            }
        }
        result
    }

    /// Function is private to not allow accidental creation of some random AIG.
    fn new() -> Self {
        Self {
            /// the first 5 fields must be initialized later, set them to max to notice if there is a bug
            maximum_variable_index: u32::MAX,
            number_of_inputs: u32::MAX,
            number_of_latches: u32::MAX,
            number_of_outputs: u32::MAX,
            number_of_and_gates: u32::MAX,

            /// the next 4 may exist or not, their default value is 0
            number_of_bad_state_constraints: 0,
            number_of_invariant_constraints: 0,
            number_of_justice_constraints: 0,
            number_of_fairness_constraints: 0,

            /// the following vectors have default lengths.
            aig_nodes: Vec::new(),
            vPis: Vec::new(),
            vPos: Vec::new(),
            vCis: Vec::new(),
            vCos: Vec::new(),
            vPios: Vec::new(),
            vBoxes: Vec::new(),
        }
    }

    fn check_first_line_of_aig_and_load_it(&mut self, lines: &[Vec<u8>]) {
        let first_line_as_str = std::str::from_utf8(&lines[0]).unwrap();
        let params: Vec<&str> = first_line_as_str.split(' ').collect();

        // check if the input file format is correct (starts with aig)
        assert_eq!(
            params[0], "aig",
            "The parameter line (first line in aig file) must start with the word 'aig'."
        );
        assert!(
            params.len() > 5,
            "The parameter line (first line in aig file) has too few arguments."
        );

        // first 5 fields always exist
        let maximum_variable_index = params[1].parse::<u32>().unwrap();
        let number_of_inputs = params[2].parse::<u32>().unwrap();
        let number_of_latches = params[3].parse::<u32>().unwrap();
        let number_of_outputs = params[4].parse::<u32>().unwrap();
        let number_of_and_gates = params[5].parse::<u32>().unwrap();

        // these fields do not always exist
        let number_of_bad_state_constraints = if params.len() > 6 {
            params[6].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_invariant_constraints = if params.len() > 7 {
            params[7].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_justice_constraints = if params.len() > 8 {
            params[8].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_fairness_constraints = if params.len() > 9 {
            params[9].parse::<u32>().unwrap()
        } else {
            0
        };
        let number_of_outputs = number_of_outputs
            + number_of_bad_state_constraints
            + number_of_invariant_constraints
            + number_of_justice_constraints
            + number_of_fairness_constraints;

        assert!(
            params.len() < 10,
            "The parameter line (first line in aig file) has too many arguments."
        );
        assert_eq!(
            maximum_variable_index,
            number_of_inputs + number_of_latches + number_of_and_gates,
            "The number of variables does not add up."
        );
        assert_eq!(
            number_of_justice_constraints, 0,
            "Reading AIGER files with liveness properties is currently not supported."
        );
        assert_eq!(
            number_of_fairness_constraints, 0,
            "Reading AIGER files with liveness properties is currently not supported."
        );

        if number_of_invariant_constraints > 0 {
            eprintln!("Warning: The last {number_of_invariant_constraints} outputs are interpreted as invariant constraints.");
        }

        self.maximum_variable_index = maximum_variable_index;
        self.number_of_inputs = number_of_inputs;
        self.number_of_latches = number_of_latches;
        self.number_of_outputs = number_of_outputs;
        self.number_of_and_gates = number_of_and_gates;

        self.number_of_bad_state_constraints = number_of_bad_state_constraints;
        self.number_of_invariant_constraints = number_of_invariant_constraints;
        self.number_of_justice_constraints = number_of_justice_constraints;
        self.number_of_fairness_constraints = number_of_fairness_constraints;
    }

    /// notice that this function does not need to read from the file since in AIG
    /// the input literals are known (2, 4, .., number_of_inputs)
    fn create_input_nodes_of_aig(&mut self) {
        assert!(self.aig_nodes.len() == 0);
        self.aig_nodes.push(AIGNode::new(AIGNodeType::ConstantZero, 0));
        for i in 0..(self.number_of_inputs) {
            let lit = 2 * (i + 1);
            let input_node = AIGNode::new(AIGNodeType::PrimaryInput, lit);
            
            // self.indexes_of_input_nodes.push(self.aig_nodes.len() as u32);
            let index = self.aig_nodes.len();
            self.aig_nodes.push(input_node);
            
            self.vPis.push(index);
            self.vCis.push(index);
        }
        assert!(self.aig_nodes.len() as u32 == (self.number_of_inputs + 1));
    }

    fn check_literal_number(&self, literal_number: u32){
        let var_number = literal_number >> 1;
        // assert!(2 <= literal_number, "'.aig' file contains literal {literal_number} which is reserved for constants.");
        assert!(var_number <= self.maximum_variable_index, "'.aig' file contains literal {literal_number} which is higher than maximum variable index.");
    }

    fn create_output_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        for i in 0..self.number_of_outputs {
            let line_number_of_output = i + 1 + self.number_of_latches;
            let line_as_vector_of_chars = &lines[line_number_of_output as usize];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();
            let output = line_as_string.parse::<u32>().unwrap();
            self.check_literal_number(output);
            
            // self.output_literals.push(output);
        }
    }

    fn create_latch_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        assert!(self.aig_nodes.len() as u32 == (self.number_of_inputs + 1));
        for i in 0..self.number_of_latches {
            // latch literal is known because this is the binary AIGER format.
            let lit = 2 * (i + self.number_of_inputs + 1);

            let line_number_of_latch = i + 1;
            let line_as_vector_of_chars = &lines[line_number_of_latch as usize];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            let mut latch_node = AIGNode::new(AIGNodeType::Latch, lit);

            if line_as_string.contains(' ') {
                // latch has a reset literal
                let parsed_line: Vec<&str> = line_as_string.split(' ').collect();
                assert!(parsed_line.len() == 2, "The latch line '{line_as_string}' does not fit the expected format.");

                let next_lit = parsed_line[0].parse::<u32>().unwrap();
                self.check_literal_number(next_lit);
                let reset = parsed_line[1].parse::<u32>().unwrap();
                assert!(reset == 0 || reset == 1 || reset == lit);

                latch_node.set_next_for_latch(next_lit);
                latch_node.set_reset_for_latch(reset);
                
            } else {
                // latch doesn't have a reset literal
                let next_lit = line_as_string.parse::<u32>().unwrap();
                self.check_literal_number(next_lit);
                
                latch_node.set_next_for_latch(next_lit);
            }

            // self.indexes_of_latch_nodes.push(self.aig_nodes.len() as u32);
            // self.aig_nodes.push(latch_node);
            let index = self.aig_nodes.len();
            self.aig_nodes.push(latch_node);
            
            self.vPis.push(index);
            self.vCis.push(index);
        }
        assert!(self.aig_nodes.len() as u32 == (self.number_of_inputs + self.number_of_latches) + 1);
    }

    

    fn create_bad_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        for i in 0..self.number_of_bad_state_constraints {
            let line_number_of_output = i + 1 + self.number_of_latches + self.number_of_outputs;
            let line_as_vector_of_chars = &lines[line_number_of_output as usize];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();
            let bad_literal = line_as_string.parse::<u32>().unwrap();
            self.check_literal_number(bad_literal);
            // self.bad_literals.push(bad_literal);
        }
    }



    fn check_aig(&self) {

        assert_eq!(self.number_of_inputs + self.number_of_latches + self.number_of_and_gates, self.maximum_variable_index);
        assert!((self.aig_nodes.len() as u32) == (self.maximum_variable_index + 1));

        // assert_eq!(self.input_nodes.len() as u32, self.number_of_inputs);
        // for node in self.input_nodes.iter() {
        //     assert!(node.get_type() == AIGNodeType::Input);
        // }

        // assert_eq!(self.latch_nodes.len() as u32, self.number_of_latches);
        // for node in self.latch_nodes.iter() {
        //     assert!(node.get_type() == AIGNodeType::Latch);
        // }
        
        // // assert_eq!(self.and_node.len() as u32, self.number_of_and_gates);


        // assert_eq!(self.output_literals.len() as u32, self.number_of_outputs);
        // assert_eq!(
        //     self.bad_literals.len() as u32,
        //     self.number_of_bad_state_constraints
        // );
        // assert_eq!(
        //     self.constraints_nodes.len() as u32,
        //     self.number_of_invariant_constraints
        // );
        // assert_eq!(
        //     self.justice_nodes.len() as u32,
        //     self.number_of_justice_constraints
        // );
        // assert_eq!(
        //     self.fairness_nodes.len() as u32,
        //     self.number_of_fairness_constraints
        // ); 
    }

    fn from_vector_of_bytes(vec_of_bytes: &Vec<u8>) -> AndInverterGraph {
        let lines = Self::split_vector_by_newline(vec_of_bytes);
        let mut aig = AndInverterGraph::new();
        aig.check_first_line_of_aig_and_load_it(&lines);
        aig.create_input_nodes_of_aig();
        aig.create_output_nodes_of_aig(&lines);
        aig.create_latch_nodes_of_aig(&lines);
        aig.create_bad_nodes_of_aig(&lines);
        aig.check_aig();
        aig
    }

    // fn load_aig_data(&mut self, vector_of_lines_as_vectors: &[Vec<u8>]) {}

    // ********************************************************************************************
    // aig api functions
    // ********************************************************************************************

    /// Function that takes path to '.aig' file and creates a corresponding AndInverterGraph object.
    /// The '.aig' file is in accordance to https://github.com/arminbiere/aiger/blob/master/FORMAT
    ///
    /// # Arguments
    ///
    /// * `file_path` - the path to the '.aig' file desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/hwmcc20_aig/2020/mann/stack-p2.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// ```
    pub fn from_aig_path(file_path: &str) -> AndInverterGraph {
        let file_as_vec_of_bytes = fs::read(file_path)
            .unwrap_or_else(|_| panic!("Unable to read the '.aig' file {file_path}"));
        Self::from_vector_of_bytes(&file_as_vec_of_bytes)
    }
}
