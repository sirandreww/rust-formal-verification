// ************************************************************************************************
// use
// ************************************************************************************************

// ************************************************************************************************
// enum
// ************************************************************************************************

#[derive(PartialEq, Debug, Clone, Copy)]
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
                       // used only for justice
                       // justice_size: usize,
                       // justice_lits: Vec<usize>,
                       // name: String,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AIGNode {
    pub fn new(lit: usize, node_type: AIGNodeType) -> Self {
        assert!(lit % 2 == 0, "Literal node should be even.");
        Self {
            lit: lit,
            latch_input: usize::MAX,
            latch_reset: usize::MAX,

            and_input0: usize::MAX,
            and_input1: usize::MAX,
            node_type: node_type,
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
}
