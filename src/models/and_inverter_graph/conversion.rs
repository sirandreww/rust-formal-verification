// ************************************************************************************************
// use
// ************************************************************************************************

use crate::models::and_inverter_graph::AndInverterGraph;

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
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
}
