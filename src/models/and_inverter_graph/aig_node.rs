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
    next: usize,
    reset: usize,

    // used only for Ands
    rhs0: usize, /* as literal [0..2*maxvar+1] */
    rhs1: usize, /* as literal [0..2*maxvar+1] */

    // used only for justice
    size: usize,
    lits: Vec<usize>,
    // name: String,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AIGNode {
    pub fn new(lit: usize, node_type: AIGNodeType) -> Self {
        Self {
            lit: lit,
            next: usize::MAX,
            reset: usize::MAX,
            size: usize::MAX,
            rhs0: usize::MAX,
            rhs1: usize::MAX,
            lits: Vec::new(),
            // name: "".to_string(),
            node_type: node_type,
        }
    }

    pub fn set_next_for_latch(&mut self, next: usize) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.next = next;
    }

    pub fn set_reset_for_latch(&mut self, reset: usize) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        assert!(reset == 0 || reset == 1 || reset == self.lit);
        self.reset = reset;
    }

    pub fn set_rhs0_for_and(&mut self, rhs0: usize) {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.rhs0 = rhs0;
    }

    pub fn set_rhs1_for_and(&mut self, rhs1: usize) {
        assert_eq!(self.node_type, AIGNodeType::And);
        self.rhs1 = rhs1;
    }

    pub fn get_type(&self) -> AIGNodeType {
        self.node_type
    }
}
