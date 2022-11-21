// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::{AIGNode, AIGNodeType};
use crate::models::and_inverter_graph::AndInverterGraph;
use std::fs;

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
        if !current_line.is_empty() {
            result.push(current_line);
        }
        result
    }

    // Function is private to not allow accidental creation of some random AIG.
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
            comments: String::from(""),
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

        assert!(
            params.len() < 10,
            "The parameter line (first line in aig file) has too many arguments."
        );

        // first 5 fields always exist
        self.maximum_variable_index = Self::convert_string_to_number(params[1]);
        self.number_of_inputs = Self::convert_string_to_number(params[2]);
        self.number_of_latches = Self::convert_string_to_number(params[3]);
        self.number_of_outputs = Self::convert_string_to_number(params[4]);
        self.number_of_and_gates = Self::convert_string_to_number(params[5]);

        // these fields do not always exist
        self.number_of_bad_state_constraints =
            Self::convert_string_to_number(params.get(6).unwrap_or(&"0"));
        self.number_of_invariant_constraints =
            Self::convert_string_to_number(params.get(7).unwrap_or(&"0"));
        self.number_of_justice_constraints =
            Self::convert_string_to_number(params.get(8).unwrap_or(&"0"));
        self.number_of_fairness_constraints =
            Self::convert_string_to_number(params.get(9).unwrap_or(&"0"));

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
                "Line {line_number_from_1}: Wrong number of arguments for latch line."
            );

            let next_lit = Self::convert_string_to_number(parsed_line[0]);
            self.check_literal(next_lit, line_number_from_1);
            self.nodes.last_mut().unwrap().set_input_of_latch(next_lit);

            if parsed_line.len() == 2 {
                // latch has a reset literal
                let reset = Self::convert_string_to_number(parsed_line[1]);
                assert!(
                    (reset == 0 || reset == 1 || reset == lit),
                    "Line {line_number_from_1}: Latch reset may be 0, 1, or equal to literal designated for latch."
                );
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
            self.constraints.push(inv_const_literal);
        }
    }

    fn get_max_literal_of_input_or_latch(&self) -> usize {
        2 * (self.number_of_inputs + self.number_of_latches)
    }

    fn get_position_of_start_of_and_segment(&self, bytes: &[u8]) -> usize {
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
        read_index
    }

    fn read_delta(&self, bytes: &[u8], mut read_index: usize) -> (usize, usize) {
        assert!(read_index < bytes.len(), "Unexpected end of file");

        let mut i: usize = 0;
        let mut delta: usize = 0;
        let mut ch: usize = bytes[read_index].into();

        while (ch & 0x80) != 0 {
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

    fn create_and_nodes_of_aig(&mut self, bytes: &[u8]) -> usize {
        let mut lhs = self.get_max_literal_of_input_or_latch();

        let mut read_index = self.get_position_of_start_of_and_segment(bytes);

        for _i in 0..self.number_of_and_gates {
            lhs += 2;
            let (delta, new_read_index) = self.read_delta(bytes, read_index);
            read_index = new_read_index;
            assert!(delta <= lhs, "Invalid delta.");
            let rhs0: usize = lhs - delta;

            let (delta, new_read_index) = self.read_delta(bytes, read_index);
            read_index = new_read_index;
            assert!(delta <= rhs0, "Invalid delta.");
            let rhs1: usize = rhs0 - delta;

            // the assert is from https://github.com/arminbiere/aiger/blob/master/FORMAT
            // line 456 as of writing this.
            assert!(
                lhs > rhs0 && rhs0 >= rhs1,
                "Error (lhs > rhs0 >= rhs1) does not hold for and gate {lhs}"
            );

            let mut node = AIGNode::new(lhs, AIGNodeType::And);
            node.set_rhs0_of_and(rhs0);
            node.set_rhs1_of_and(rhs1);
            self.ands.push(self.nodes.len());
            self.nodes.push(node);
        }

        read_index
    }

    fn add_symbol_to_node(&mut self, symbol_type: &str, symbol_number: usize, symbol: &str) {
        if symbol_type == "i" {
            let node_index = self.inputs[symbol_number];
            self.nodes[node_index].set_input_symbol(symbol);
        } else if symbol_type == "l" {
            let node_index = self.latches[symbol_number];
            self.nodes[node_index].set_latch_symbol(symbol);
        } else if symbol_type == "o" {
            let node_index = self.outputs[symbol_number] >> 1;
            self.nodes[node_index].set_output_symbol(symbol);
        } else if symbol_type == "b" {
            let node_index = self.bad[symbol_number] >> 1;
            self.nodes[node_index].set_bad_symbol(symbol);
        } else if symbol_type == "c" {
            let node_index = self.constraints[symbol_number] >> 1;
            self.nodes[node_index].set_constraint_symbol(symbol);
        } else {
            unreachable!();
        }
    }

    fn read_symbols_and_comments(
        &mut self,
        bytes: &[u8],
        position_of_end_of_and_segment_plus_one: usize,
    ) {
        // position_of_end_of_and_segment_plus_one == position where symbol table might begin
        let lines: &[Vec<u8>] =
            &Self::split_vector_by_newline(&bytes[position_of_end_of_and_segment_plus_one..]);
        for (index, line_as_vector_of_chars) in lines.iter().enumerate() {
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            if line_as_string == "c" {
                // comment segment started, we can read this till the end and return
                let rest_of_comments = &lines[index..].join(&b'\n');
                let comment_section_as_is =
                    std::str::from_utf8(&rest_of_comments).unwrap().to_string();
                self.comments = comment_section_as_is.replace(&char::from(0).to_string(), "");
                break;
            } else {
                let parsed_line: Vec<&str> = line_as_string.split(' ').collect();
                assert!(
                    parsed_line.len() == 2,
                    "Line '{line_as_string}': Wrong number of arguments for symbol line."
                );
                let mut symbol_and_variable_split: Vec<&str> = parsed_line[0].split("").collect();
                // "i0" gets split into vec!["" , "i", "0", ""], let's drop start and end.
                symbol_and_variable_split =
                    symbol_and_variable_split[1..(symbol_and_variable_split.len() - 1)].to_vec();
                assert!(
                    symbol_and_variable_split.len() > 1,
                    "Line '{line_as_string}': Symbol line should start with [ilobc]<pos>."
                );

                let symbol_type = symbol_and_variable_split[0];
                assert!(
                    ["i", "l", "o", "b", "c"].contains(&symbol_type),
                    "Line '{line_as_string}': Symbol line should start with [ilobc]<pos>."
                );
                let var_as_vector_of_strings = symbol_and_variable_split[1..].to_vec();
                let symbol_number_as_string = var_as_vector_of_strings.join("");
                let symbol_number = Self::convert_string_to_number(&symbol_number_as_string);
                self.add_symbol_to_node(symbol_type, symbol_number, parsed_line[1])
            }
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

    fn from_vector_of_bytes(vec_of_bytes: &[u8]) -> AndInverterGraph {
        let lines = Self::split_vector_by_newline(vec_of_bytes);
        let mut aig = AndInverterGraph::new();
        aig.check_first_line_of_aig_and_load_it(&lines);
        aig.allocate_vectors();
        aig.create_input_nodes_of_aig();
        aig.create_latch_nodes_of_aig(&lines);
        aig.create_output_nodes_of_aig(&lines);
        aig.create_bad_nodes_of_aig(&lines);
        aig.create_invariant_constraint_nodes_of_aig(&lines);
        let position_of_end_of_and_segment_plus_one = aig.create_and_nodes_of_aig(vec_of_bytes);
        aig.read_symbols_and_comments(vec_of_bytes, position_of_end_of_and_segment_plus_one);
        aig.check_aig();
        aig
    }

    // ********************************************************************************************
    // aig creator
    // ********************************************************************************************

    /// Function that takes path to '.aig' file and creates a corresponding AndInverterGraph object.
    /// The '.aig' file is in accordance to http://fmv.jku.at/aiger/
    ///
    /// # Arguments
    ///
    /// * `file_path` - the path to the '.aig' file desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/hwmcc20/2020/mann/stack-p2.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// ```
    pub fn from_aig_path(file_path: &str) -> AndInverterGraph {
        let file_as_vec_of_bytes = fs::read(file_path)
            .unwrap_or_else(|_| panic!("Unable to read the '.aig' file {file_path}"));
        Self::from_vector_of_bytes(&file_as_vec_of_bytes)
    }
}
