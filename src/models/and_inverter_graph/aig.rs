// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::{AIGNode, AIGNodeType};
use std::{fs, result};

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct AndInverterGraph {
    maximum_variable_index: usize,
    number_of_inputs: usize,
    number_of_latches: usize,
    number_of_outputs: usize,
    number_of_and_gates: usize,
    number_of_bad_state_constraints: usize,
    number_of_invariant_constraints: usize,
    number_of_justice_constraints: usize,
    number_of_fairness_constraints: usize,

    nodes: Vec<AIGNode>, /* [0..maxvar] */

    // these contain indexes that are in nodes that have these nodes.
    inputs: Vec<usize>,  /* [0..num_inputs] */
    latches: Vec<usize>, /* [0..num_latches] */
    ands: Vec<usize>,    /* [0..num_ands] */

    // these contain literals.
    outputs: Vec<usize>, /* [0..num_outputs] */
    bad: Vec<usize>,     /* [0..num_bad] */
    constraints: Vec<usize>, /* [0..num_constraints] */
                         // justice: Vec<usize>,     /* [0..num_justice] */
                         // fairness: Vec<usize>,    /* [0..num_fairness] */
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
        if current_line.len() > 0 {
            result.push(current_line);
        }
        result
    }

    /// Function is private to not allow accidental creation of some random AIG.
    fn new() -> Self {
        Self {
            /// these fields must be changed later, set them to max to notice if there is a bug
            maximum_variable_index: usize::MAX,
            number_of_inputs: usize::MAX,
            number_of_latches: usize::MAX,
            number_of_outputs: usize::MAX,
            number_of_and_gates: usize::MAX,
            number_of_bad_state_constraints: usize::MAX,
            number_of_invariant_constraints: usize::MAX,
            number_of_justice_constraints: usize::MAX,
            number_of_fairness_constraints: usize::MAX,

            /// the following vectors have default lengths.
            nodes: Vec::new(),
            inputs: Vec::new(),
            latches: Vec::new(),
            outputs: Vec::new(),
            ands: Vec::new(),
            bad: Vec::new(),
            constraints: Vec::new(),
            // justice: Vec::new(),
            // fairness: Vec::new(),
        }
    }

    fn convert_string_to_number(str1: &str) -> usize {
        str1.parse::<usize>().unwrap()
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
        self.maximum_variable_index = Self::convert_string_to_number(params[1]);
        self.number_of_inputs = Self::convert_string_to_number(params[2]);
        self.number_of_latches = Self::convert_string_to_number(params[3]);
        self.number_of_outputs = Self::convert_string_to_number(params[4]);
        self.number_of_and_gates = Self::convert_string_to_number(params[5]);

        // these fields do not always exist
        self.number_of_bad_state_constraints = if params.len() > 6 {
            Self::convert_string_to_number(params[6])
        } else {
            0
        };
        self.number_of_invariant_constraints = if params.len() > 7 {
            Self::convert_string_to_number(params[7])
        } else {
            0
        };
        self.number_of_justice_constraints = if params.len() > 8 {
            Self::convert_string_to_number(params[8])
        } else {
            0
        };
        self.number_of_fairness_constraints = if params.len() > 9 {
            Self::convert_string_to_number(params[9])
        } else {
            0
        };

        assert!(
            params.len() < 10,
            "The parameter line (first line in aig file) has too many arguments."
        );
        assert_eq!(
            self.maximum_variable_index,
            self.number_of_inputs + self.number_of_latches + self.number_of_and_gates,
            "The number of variables does not add up."
        );
        assert_eq!(
            self.number_of_fairness_constraints, 0,
            "Fairness is currently unsupported."
        );
        assert_eq!(
            self.number_of_justice_constraints, 0,
            "Justice is currently unsupported."
        );
        // assert_eq!(self.number_of_outputs, 0, "Output is currently unsupported.");
    }

    fn allocate_vectors(&mut self) {
        self.nodes = Vec::with_capacity(self.maximum_variable_index + 1);
        self.nodes.push(AIGNode::new(0, AIGNodeType::ConstantZero));

        self.inputs = Vec::with_capacity(self.number_of_inputs);
        self.latches = Vec::with_capacity(self.number_of_latches);
        self.outputs = Vec::with_capacity(self.number_of_outputs);
        self.ands = Vec::with_capacity(self.number_of_and_gates);
        self.bad = Vec::with_capacity(self.number_of_bad_state_constraints);
        self.constraints = Vec::with_capacity(self.number_of_invariant_constraints);
        // self.justice = Vec::with_capacity(self.number_of_justice_constraints);
        // self.fairness = Vec::with_capacity(self.number_of_fairness_constraints);
    }

    /// notice that this function does not need to read from the file since in AIG
    /// the input literals are known (2, 4, .., 2 * number_of_inputs)
    fn create_input_nodes_of_aig(&mut self) {
        // assert!(self.aig_nodes.len() == 0);
        for i in 0..(self.number_of_inputs) {
            let lit = 2 * (i + 1);
            self.inputs.push(self.nodes.len());
            self.nodes.push(AIGNode::new(lit, AIGNodeType::Input));
        }
    }

    fn check_literal(&self, literal_number: usize, line_num: usize) {
        let var_number = literal_number >> 1;
        // assert!(2 <= literal_number, "Line {line_num}: '.aig' file contains literal {literal_number} which is reserved for constants.");
        assert!(var_number <= self.maximum_variable_index, "Line {line_num}: '.aig' file contains literal {literal_number} which is higher than maximum variable index.");
    }

    fn create_latch_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        for i in 0..self.number_of_latches {
            // latch literal is known because this is the binary AIGER format.
            let lit = 2 * (i + self.number_of_inputs + 1);
            self.latches.push(self.nodes.len());
            self.nodes.push(AIGNode::new(lit, AIGNodeType::Latch));

            let line_number_from_0: usize = i + 1;
            let line_number_from_1: usize = line_number_from_0 + 1;
            let line_as_vector_of_chars = &lines[line_number_from_0];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            let parsed_line: Vec<&str> = line_as_string.split(' ').collect();
            assert!(
                parsed_line.len() == 1 || parsed_line.len() == 2,
                "Line {line_number_from_1}: Wrong number of arguments."
            );

            let next_lit = Self::convert_string_to_number(parsed_line[0]);
            self.check_literal(next_lit, line_number_from_1);
            self.nodes.last_mut().unwrap().set_input_of_latch(next_lit);

            if parsed_line.len() == 2 {
                // latch has a reset literal
                let reset = Self::convert_string_to_number(parsed_line[1]);
                assert!(reset == 0 || reset == 1 || reset == lit);
                self.nodes.last_mut().unwrap().set_reset_of_latch(reset);
            } else {
                // latch does not have a reset literal (defaults to 0)
                // https://epub.jku.at/obvulioa/content/titleinfo/5973560/full.pdf
                self.nodes.last_mut().unwrap().set_reset_of_latch(0);
            }
        }
    }

    fn create_output_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        for i in 0..self.number_of_outputs {
            let line_number_from_0: usize = i + 1 + self.number_of_latches;
            let line_number_from_1: usize = line_number_from_0 + 1;

            let line_as_vector_of_chars = &lines[line_number_from_0];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            let output_literal = Self::convert_string_to_number(line_as_string);
            self.check_literal(output_literal, line_number_from_1);
            assert!(
                self.outputs.contains(&output_literal) == false,
                "Line {line_number_from_1}: Output is repeated twice."
            );
            self.outputs.push(output_literal);
        }
    }

    fn create_bad_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        // println!("Bad:");
        for i in 0..self.number_of_bad_state_constraints {
            let line_number_from_0: usize = i + 1 + self.number_of_latches + self.number_of_outputs;
            let line_number_from_1: usize = line_number_from_0 + 1;

            let line_as_vector_of_chars = &lines[line_number_from_0];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            let bad_literal = Self::convert_string_to_number(line_as_string);
            // println!("{bad_literal}");
            self.check_literal(bad_literal, line_number_from_1);
            assert!(
                self.bad.contains(&bad_literal) == false,
                "Line {line_number_from_1}: Bad is repeated twice."
            );
            self.bad.push(bad_literal);
        }
    }

    fn create_invariant_constraint_nodes_of_aig(&mut self, lines: &[Vec<u8>]) {
        for i in 0..self.number_of_invariant_constraints {
            let line_number_from_0: usize = i
                + 1
                + self.number_of_latches
                + self.number_of_outputs
                + self.number_of_bad_state_constraints;
            let line_number_from_1: usize = line_number_from_0 + 1;

            let line_as_vector_of_chars = &lines[line_number_from_0];
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            let inv_const_literal = Self::convert_string_to_number(line_as_string);
            self.check_literal(inv_const_literal, line_number_from_1);
            // assert!(self.constraints.contains(&inv_const_literal) == false, "Line {line_number_from_1}: Constraint is repeated twice.");
            self.constraints.push(inv_const_literal);
        }
    }

    fn get_max_literal_of_input_or_latch(&self) -> usize {
        return 2 * (self.number_of_inputs + self.number_of_latches);
    }

    fn read_delta(&self, bytes: &[u8], mut read_index: usize) -> (usize, usize) {
        assert!(read_index < bytes.len(), "Unexpected end of file");

        let mut i: usize = 0;
        let mut delta: usize = 0;
        let mut ch: usize = bytes[read_index].into();

        while ((ch & 0x80) != 0) {
            assert_ne!(i, 5, "Invalid code");

            delta |= (ch & 0x7f) << (7 * i);
            i += 1;
            read_index += 1;
            ch = bytes[read_index].into();

            assert!(read_index < bytes.len(), "Unexpected end of file");
        }
        assert!(i != 5 || ch < 8);
        delta |= ch << (7 * i);
        (delta, (read_index + 1))
    }

    fn create_and_nodes_of_aig(&mut self, bytes: &[u8]) {
        let mut lhs = self.get_max_literal_of_input_or_latch();

        let mut read_index: usize = 0;
        let amount_of_lines_to_skip: usize = 1
            + self.number_of_latches
            + self.number_of_outputs
            + self.number_of_bad_state_constraints
            + self.number_of_invariant_constraints;
        let mut new_lines_seen = 0;
        while new_lines_seen < amount_of_lines_to_skip {
            if bytes[read_index] == b'\n' {
                new_lines_seen += 1;
            }
            read_index += 1;
        }

        for i in 0..self.number_of_and_gates {
            lhs += 2;
            let (delta, new_read_index) = self.read_delta(bytes, read_index);
            read_index = new_read_index;
            assert!(delta <= lhs, "Invalid delta.");
            let rhs0: usize = lhs - delta;

            let (delta, new_read_index) = self.read_delta(bytes, read_index);
            read_index = new_read_index;
            assert!(delta <= rhs0, "Invalid delta.");
            let rhs1 :usize= rhs0 - delta;

            let mut node = AIGNode::new(lhs, AIGNodeType::And);
            node.set_rhs0_of_and(rhs0);
            node.set_rhs1_of_and(rhs1);
            self.ands.push(self.nodes.len());
            self.nodes.push(node);


            // let inv_const_literal = Self::convert_string_to_number(line_as_string);
            // self.check_literal(inv_const_literal, line_number_from_1);
            // assert!(self.constraints.contains(&inv_const_literal) == false, "Line {line_number_from_1}: Constraint is repeated twice.");
        }
    }

    fn check_aig(&self) {
        assert_eq!(self.nodes[0].get_type(), AIGNodeType::ConstantZero);
        // inputs
        assert_eq!(self.number_of_inputs, self.inputs.len());
        for input_index in &self.inputs {
            let i = input_index.to_owned();
            assert_eq!(self.nodes[i].get_type(), AIGNodeType::Input);
        }
        // latches
        assert_eq!(self.number_of_latches, self.latches.len());
        for latch_index in &self.latches {
            let i = latch_index.to_owned();
            assert_eq!(self.nodes[i].get_type(), AIGNodeType::Latch);
        }
        // ands
        assert_eq!(self.number_of_and_gates, self.ands.len());
        for and_index in &self.ands {
            let i = and_index.to_owned();
            assert_eq!(self.nodes[i].get_type(), AIGNodeType::And);
        }
        assert_eq!(self.number_of_outputs, self.outputs.len());
        assert_eq!(self.number_of_bad_state_constraints, self.bad.len());
        assert_eq!(self.number_of_invariant_constraints, self.constraints.len());
    }

    fn from_vector_of_bytes(vec_of_bytes: &Vec<u8>) -> AndInverterGraph {
        let lines = Self::split_vector_by_newline(vec_of_bytes);
        let mut aig = AndInverterGraph::new();
        aig.check_first_line_of_aig_and_load_it(&lines);
        aig.allocate_vectors();
        aig.create_input_nodes_of_aig();
        aig.create_latch_nodes_of_aig(&lines);
        aig.create_output_nodes_of_aig(&lines);
        aig.create_bad_nodes_of_aig(&lines);
        aig.create_invariant_constraint_nodes_of_aig(&lines);
        aig.create_and_nodes_of_aig(&vec_of_bytes);
        aig.check_aig();
        aig
    }

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

    pub fn get_aag_string(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        let mut first_line = vec![String::from("aag"), ];
        first_line.push(self.maximum_variable_index.to_string());
        first_line.push(self.number_of_inputs.to_string());
        first_line.push(self.number_of_latches.to_string());
        first_line.push(self.number_of_outputs.to_string());
        first_line.push(self.number_of_and_gates.to_string());
        assert!(self.number_of_justice_constraints + self.number_of_fairness_constraints == 0);
        if self.number_of_bad_state_constraints + self.number_of_invariant_constraints > 0 {
            first_line.push(self.number_of_bad_state_constraints.to_string());
        }
        if self.number_of_invariant_constraints > 0 {
            first_line.push(self.number_of_invariant_constraints.to_string());
        }
        result.push(first_line.join(" "));
        for input_index in &self.inputs {
            result.push(self.nodes[input_index.to_owned()].get_literal().to_string());
        }
        for latch_index in &self.latches {
            let mut line = Vec::new();
            let node = &self.nodes[latch_index.to_owned()];
            line.push(node.get_literal().to_string());
            line.push(node.get_latch_input().to_string());
            if node.get_latch_reset() != 0 {
                line.push(node.get_latch_reset().to_string());
            }
            result.push(line.join(" "));
        }
        for output_literal in &self.outputs {
            result.push(output_literal.to_string());
        }
        for bad_literal in &self.bad {
            result.push(bad_literal.to_string());
        }
        for constraint_literal in &self.constraints {
            result.push(constraint_literal.to_string());
        }
        for and_index in &self.ands{
            let node = &self.nodes[and_index.to_owned()];
            let lhs = node.get_literal();
            let rhs0 = node.get_and_rhs0();
            let rhs1 = node.get_and_rhs1();
            result.push(format!("{lhs} {rhs0} {rhs1}"));
        }

        return result.join("\n");
    }
}
