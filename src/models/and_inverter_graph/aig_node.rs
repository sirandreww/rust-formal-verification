// ************************************************************************************************
// use
// ************************************************************************************************

// ************************************************************************************************
// enum
// ************************************************************************************************

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) enum AIGNodeType {
    ConstantZero,
    PrimaryInput, 
    PrimaryOutput,
    BoxInput,       
    BoxOutput,         
    Net,       
    Node,       
    Latch,    
}

// ************************************************************************************************
// struct
// ************************************************************************************************

pub(crate) struct AIGNode {
    node_type: AIGNodeType,

    // as literal [0..2*maxvar+1]
    literal_number: u32,

    // used only for latches
    next: u32,
    reset: u32,

    // used only for justice
    // size: u32,
    // lits: Vec<u32>,

    // name: String,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AIGNode {
    pub(crate) fn new(node_type: AIGNodeType, literal_number: u32) -> Self {
        if node_type == AIGNodeType::ConstantZero {
            assert!(literal_number == 0);
        }
        Self {
            node_type: node_type,
            literal_number: literal_number,
            next: u32::MAX,
            reset: u32::MAX,
            // size: u32::MAX,
            // lits: Vec::new(),
            // name: String::from(""),
        }
    }

    pub(crate) fn set_next_for_latch(&mut self, next: u32) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        self.next = next;
    }

    pub(crate) fn set_reset_for_latch(&mut self, rest: u32) {
        assert_eq!(self.node_type, AIGNodeType::Latch);
        assert!(rest == 0 || rest == 1 || rest == self.literal_number);
        self.reset = rest;
    }

    pub(crate) fn get_type(&self) -> AIGNodeType {
        self.node_type
    }
}
