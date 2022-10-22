// ************************************************************************************************
// use
// ************************************************************************************************

// ************************************************************************************************
// enum
// ************************************************************************************************

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AIGNodeType {
    ConstantZero,
    Input,
    Latch,
    And,
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub struct AIGNode {
    node_type: AIGNodeType,

    // as literal [0..2*maxvar+1]
    lit: usize,

    // used only for latches
    latch_input: usize,
    latch_reset: usize,

    // used only for Ands
    and_input0: usize, /* as literal [0..2*maxvar+1] */
    and_input1: usize, /* as literal [0..2*maxvar+1] */
    
    // symbols
    input_symbol: String,
    latch_symbol: String,
    output_symbol: String,
    bad_symbol: String,
    constraint_symbol: String,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AIGNode {
    pub fn new(lit: usize, node_type: AIGNodeType) -> Self {
        assert!(lit % 2 == 0, "Literal node should be even.");
        Self {
            lit,
            latch_input: usize::MAX,
            latch_reset: usize::MAX,

            and_input0: usize::MAX,
            and_input1: usize::MAX,
            node_type,

            input_symbol: String::from(""),
            latch_symbol: String::from(""),
            output_symbol: String::from(""),
            bad_symbol: String::from(""),
            constraint_symbol: String::from(""),
        }
    }

    pub fn set_input_of_latch(&mut self, input: usize) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.latch_input = input;
    }

    pub fn set_reset_of_latch(&mut self, reset: usize) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        assert!(reset == 0 || reset == 1 || reset == self.lit);
        self.latch_reset = reset;
    }

    pub fn set_rhs0_of_and(&mut self, rhs0: usize) {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.and_input0 = rhs0;
    }

    pub fn set_rhs1_of_and(&mut self, rhs1: usize) {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.and_input1 = rhs1;
    }

    pub fn get_type(&self) -> AIGNodeType {
        self.node_type
    }

    pub fn get_literal(&self) -> usize {
        self.lit
    }

    pub fn get_latch_input(&self) -> usize {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.latch_input
    }

    pub fn get_latch_reset(&self) -> usize {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.latch_reset
    }

    pub fn get_and_rhs0(&self) -> usize {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.and_input0
    }

    pub fn get_and_rhs1(&self) -> usize {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.and_input1
    }

    pub fn set_input_symbol(&mut self, symbol: &str) {
        assert_eq!(self.node_type, AIGNodeType::Input);
        self.input_symbol = symbol.to_string();
    }

    pub fn get_input_symbol(&self) -> &str {
        assert_eq!(self.node_type, AIGNodeType::Input);
        self.input_symbol.as_str()
    }

    pub fn set_latch_symbol(&mut self, symbol: &str) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.latch_symbol = symbol.to_string();
    }

    pub fn get_latch_symbol(&self) -> &str {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.latch_symbol.as_str()
    }

    pub fn set_output_symbol(&mut self, symbol: &str) {
        self.output_symbol = symbol.to_string();
    }

    pub fn get_output_symbol(&self) -> &str {
        self.output_symbol.as_str()
    }

    pub fn set_bad_symbol(&mut self, symbol: &str) {
        self.bad_symbol = symbol.to_string();
    }

    pub fn get_bad_symbol(&self) -> &str {
        self.bad_symbol.as_str()
    }

    pub fn set_constraint_symbol(&mut self, symbol: &str){
        self.constraint_symbol = symbol.to_string();
    }

    pub fn get_constraint_symbol(&self) -> &str {
        self.constraint_symbol.as_str()
    }
}
