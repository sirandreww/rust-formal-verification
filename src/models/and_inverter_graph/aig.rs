// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::aig_node::{AIGNode, AIGNodeType};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

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
    inputs: Vec<usize>,
    latches: Vec<usize>,
    ands: Vec<usize>,

    // these contain literals.
    outputs: Vec<usize>,
    bad: Vec<usize>,
    constraints: Vec<usize>,
}

// ************************************************************************************************
// AIG simulation result
// ************************************************************************************************

pub struct AIGSimulationResult {
    states: Vec<Vec<bool>>,
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
        for line_as_vector_of_chars in lines.iter() {
            let line_as_string = std::str::from_utf8(line_as_vector_of_chars).unwrap();

            if line_as_string == "c" {
                // comment segment started, we can stop reading
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

    fn get_pre_initial_simulation_state(
        &self,
        initial_input: &HashMap<usize, bool>,
        initial_latches: &HashMap<usize, bool>,
    ) -> Vec<bool> {
        let mut result = vec![false; self.nodes.len()];
        for (input, value) in initial_input.iter() {
            result[input.to_owned()] = value.to_owned();
        }
        for (latch, value) in initial_latches.iter() {
            let node = &self.nodes[latch.to_owned()];
            let latch_input = node.get_latch_input();
            if latch_input % 2 == 0 {
                result[latch_input >> 1] = value.to_owned();
            } else {
                result[latch_input >> 1] = !value.to_owned();
            }
        }
        assert_eq!(result.len(), self.nodes.len());
        result
    }

    fn get_next_simulation_state(
        &self,
        last_state: &Vec<bool>,
        current_input: &HashMap<usize, bool>,
    ) -> Vec<bool> {
        let mut result = Vec::new();
        for aig_node in self.nodes.iter() {
            let var_num = aig_node.get_literal() >> 1;
            assert_eq!(result.len(), var_num);
            match aig_node.get_type() {
                AIGNodeType::ConstantZero => {
                    assert_eq!(result.len(), 0);
                    result.push(false);
                }
                AIGNodeType::Input => {
                    result.push(current_input[&var_num]);
                }
                AIGNodeType::Latch => {
                    let latch_in = aig_node.get_latch_input();
                    result.push(if latch_in % 2 == 0 {
                        last_state[&latch_in >> 1]
                    } else {
                        !last_state[&latch_in >> 1]
                    });
                }
                AIGNodeType::And => {
                    let rhs0 = aig_node.get_and_rhs0();
                    let rhs1 = aig_node.get_and_rhs1();
                    let rhs0_value = if rhs0 % 2 == 0 {
                        result[rhs0 >> 1]
                    } else {
                        !result[rhs0 >> 1]
                    };
                    let rhs1_value = if rhs1 % 2 == 0 {
                        result[rhs1 >> 1]
                    } else {
                        !result[rhs1 >> 1]
                    };
                    result.push(rhs0_value && rhs1_value);
                }
            }
        }
        assert_eq!(result.len(), self.nodes.len());
        result
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

    // ********************************************************************************************
    // aig converter
    // ********************************************************************************************

    /// Function that converts an AndInverterGraph into '.aag' format as described in:
    /// The '.aag' file is in accordance to http://fmv.jku.at/aiger/
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired for conversion.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!("aag 5 0 3 1 2\n2 10\n4 2\n6 4\n10\n8 7 5\n10 8 3\n", aig.get_aag_string());
    /// ```
    pub fn get_aag_string(&self) -> String {
        let mut result: Vec<String> = Vec::new();
        let mut symbol_table: Vec<String> = Vec::new();
        let mut first_line = vec![String::from("aag")];
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
        for (index, input_index) in self.inputs.iter().enumerate() {
            let node = &self.nodes[input_index.to_owned()];
            result.push(node.get_literal().to_string());
            let symbol = node.get_input_symbol();
            if !symbol.is_empty() {
                symbol_table.push(format!("i{index} {symbol}"));
            }
        }
        for (index, latch_index) in self.latches.iter().enumerate() {
            let mut line = Vec::new();
            let node = &self.nodes[latch_index.to_owned()];
            line.push(node.get_literal().to_string());
            line.push(node.get_latch_input().to_string());
            if node.get_latch_reset() != 0 {
                line.push(node.get_latch_reset().to_string());
            }
            result.push(line.join(" "));
            let symbol = node.get_latch_symbol();
            if !symbol.is_empty() {
                symbol_table.push(format!("l{index} {symbol}"));
            }
        }
        for (index, output_literal) in self.outputs.iter().enumerate() {
            result.push(output_literal.to_string());

            let node = &self.nodes[(output_literal >> 1).to_owned()];
            let symbol = node.get_output_symbol();
            if !symbol.is_empty() {
                symbol_table.push(format!("o{index} {symbol}"));
            }
        }
        for (index, bad_literal) in self.bad.iter().enumerate() {
            result.push(bad_literal.to_string());

            let node = &self.nodes[(bad_literal >> 1).to_owned()];
            let symbol = node.get_bad_symbol();
            if !symbol.is_empty() {
                symbol_table.push(format!("b{index} {symbol}"));
            }
        }
        for (index, constraint_literal) in self.constraints.iter().enumerate() {
            result.push(constraint_literal.to_string());

            let node = &self.nodes[(constraint_literal >> 1).to_owned()];
            let symbol = node.get_constraint_symbol();
            if !symbol.is_empty() {
                symbol_table.push(format!("c{index} {symbol}"));
            }
        }
        for and_index in &self.ands {
            let node = &self.nodes[and_index.to_owned()];
            let lhs = node.get_literal();
            let rhs0 = node.get_and_rhs0();
            let rhs1 = node.get_and_rhs1();
            result.push(format!("{lhs} {rhs0} {rhs1}"));
        }
        result.append(&mut symbol_table);
        let mut final_res = result.join("\n");
        final_res.push('\n');
        final_res
    }

    // ********************************************************************************************
    // aig getting node info
    // ********************************************************************************************

    /// Function that gets a vector describing the input nodes in the system.
    /// The output is a vector containing number representing input literals.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_input_information());
    /// ```
    pub fn get_input_information(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for input_index in self.inputs.iter() {
            let input = &self.nodes[input_index.to_owned()];
            result.push(input.get_literal());
        }
        result
    }

    /// Function that gets a vector describing the latch nodes in the system.
    /// The output is a vector containing tuple with a length of 3,
    /// representing latch information :
    /// ```
    /// // (latch output literal, latch input literal, latch initial value)
    /// //                   ___________
    /// //                  |           |
    /// // latch input ---> |   latch   | --> latch output
    /// //                  |___________|
    /// //                        ^
    /// //                        |
    /// //               latch initial value
    /// ```
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(vec![(2, 10, 0),(4, 2, 0),(6, 4, 0)], aig.get_latch_information());
    /// ```
    pub fn get_latch_information(&self) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        for latch_index in self.latches.iter() {
            let latch = &self.nodes[latch_index.to_owned()];

            let latch_literal = latch.get_literal();
            let latch_input = latch.get_latch_input();
            let latch_reset = latch.get_latch_reset();

            result.push((latch_literal, latch_input, latch_reset));
        }
        result
    }

    // ********************************************************************************************
    // aig getting special literals
    // ********************************************************************************************

    /// Function that gets a vector describing the bad nodes in the system.
    /// The output is a vector containing usize numbers, these are the literals
    /// that are bad.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_bad_information());
    /// ```
    pub fn get_bad_information(&self) -> Vec<usize> {
        self.bad.clone()
    }

    /// Function that gets a vector describing the constraints nodes in the system.
    /// The output is a vector containing usize numbers, these are the literals
    /// that are constraint.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(Vec::<usize>::new(), aig.get_constraints_information());
    /// ```
    pub fn get_constraints_information(&self) -> Vec<usize> {
        self.constraints.clone()
    }

    // ********************************************************************************************
    // aig getting numbers
    // ********************************************************************************************

    /// Function that gets the maximum variable number used in the AIG.
    ///
    /// # Arguments
    ///
    /// * `&self` - the AndInverterGraph desired.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(5, aig.get_highest_variable_number());
    /// ```
    pub fn get_highest_variable_number(&self) -> usize {
        self.maximum_variable_index
    }

    // ********************************************************************************************
    // aig getting and gates
    // ********************************************************************************************

    /// Function that gets the cone of influence for some list of literals.
    ///
    /// # Arguments
    ///
    /// * `&self` - The AndInverterGraph desired.
    /// * `desired_literals: &[usize]` - The literals you want the AND gates for.
    ///
    /// # Examples
    /// ```
    /// use rust_formal_verification::models::AndInverterGraph;
    /// let file_path = "tests/examples/ours/counter.aig";
    /// // the aig looks like this:
    /// // *--------------------------------------------------------------*
    /// // |     _________                                                |
    /// // |    |         |                                               |
    /// // *--> | latch 0 |  2-*                                          |
    /// //      |_________|    |                                          |
    /// //                     |                                          |
    /// // *-------------------*-------------Not-------*                  |
    /// // |     _________                             |     ______       |
    /// // |    |         |                 ______     *--> |      \      |
    /// // *--> | latch 1 |  4-*---Not---> |      \         |  and  ) 10--*
    /// //      |_________|    |           |  and  )  8---> |______/
    /// //                     |     *---> |______/
    /// // *-------------------*     |
    /// // |     _________          Not
    /// // |    |         |          |
    /// // *--> | latch 2 |  6-------*
    /// //      |_________|
    /// let aig = AndInverterGraph::from_aig_path(file_path);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[0]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[1]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[2]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[3]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[4]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[5]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[6]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[7]), vec![]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[8]), vec![(8, 7, 5)]);
    /// assert_eq!(aig.get_and_information_in_cone_of_influence(&[9]), vec![(8, 7, 5)]);
    /// let mut sorted = aig.get_and_information_in_cone_of_influence(&[10]);
    /// sorted.sort();
    /// assert_eq!(sorted, vec![(8, 7, 5), (10, 8, 3)]);
    /// let mut sorted = aig.get_and_information_in_cone_of_influence(&[11]);
    /// sorted.sort();
    /// assert_eq!(sorted, vec![(8, 7, 5), (10, 8, 3)]);
    /// ```
    pub fn get_and_information_in_cone_of_influence(
        &self,
        desired_literals: &[usize],
    ) -> Vec<(usize, usize, usize)> {
        let mut and_gates: HashSet<(usize, usize, usize)> = HashSet::new();
        assert!(and_gates.is_empty());
        let mut current_wanted_literals = desired_literals.to_owned();
        let mut i = 0;
        while i < current_wanted_literals.len() {
            let node = &self.nodes[current_wanted_literals[i] >> 1];
            match node.get_type() {
                AIGNodeType::ConstantZero => {}
                AIGNodeType::Input => {}
                AIGNodeType::Latch => {}
                AIGNodeType::And => {
                    let and_out = node.get_literal();
                    let in0 = node.get_and_rhs0();
                    let in1 = node.get_and_rhs1();
                    if and_gates.contains(&(and_out, in0, in1)) {
                        // already visited this AND gate
                    } else {
                        // haven't seen this AND gate before
                        and_gates.insert((and_out, in0, in1));
                        current_wanted_literals.push(in0);
                        current_wanted_literals.push(in1);
                    }
                }
            };
            i += 1;
        }
        Vec::from_iter(and_gates)
    }

    // ********************************************************************************************
    // aig getting and gates
    // ********************************************************************************************

    pub fn simulate(
        &self,
        inputs: &Vec<HashMap<usize, bool>>,
        initial_latches: &HashMap<usize, bool>,
    ) -> AIGSimulationResult {
        // check inputs
        assert!(
            inputs.len() > 0,
            "Inputs cannot be empty to start simulation."
        );
        for clk_inputs in inputs {
            assert_eq!(clk_inputs.len(), self.number_of_inputs);
            assert_eq!(self.inputs.len(), self.number_of_inputs);
            // check that each clock has the correct var numbers.
            for input_var in self.inputs.iter() {
                assert!(clk_inputs.contains_key(&input_var));
            }
        }
        // check initial latches
        assert!(
            initial_latches.len() <= self.number_of_latches,
            "Too many initial latches provided."
        );
        assert_eq!(self.latches.len(), self.number_of_latches);
        for latch_var in self.latches.iter() {
            assert!(initial_latches.contains_key(&latch_var));
        }

        // prepare result
        let mut result = AIGSimulationResult { states: Vec::new() };

        let mut last_state = self.get_pre_initial_simulation_state(&inputs[0], initial_latches);
        println!("State -1 = {:?}", last_state);
        for clk_number in 0..inputs.len() {
            let current_input = &inputs[clk_number];
            let next_state = self.get_next_simulation_state(&last_state, current_input);
            last_state = next_state.to_owned();
            result.states.push(next_state);
            println!("State {} = {:?}", clk_number, last_state);
        }
        result
    }
}
