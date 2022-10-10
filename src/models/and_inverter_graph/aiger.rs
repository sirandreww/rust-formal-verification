// ************************************************************************************************
// use
// ************************************************************************************************

use std::fs;

// ************************************************************************************************
// struct
// ************************************************************************************************

// This implementation is in accordance to https://github.com/arminbiere/aiger/blob/master/FORMAT
#[derive(Default)]
pub struct AndInverterGraph {
    maximum_variable_index: u32,
    number_of_inputs: u32,
    number_of_latches: u32,
    number_of_outputs: u32,
    number_of_and_gates: u32,
    number_of_bad_outputs: u32,
    number_of_constraint_outputs: u32,
    number_of_justice_outputs: u32,
    number_of_fairness_outputs: u32,
    /*
        // general information
        Abc_NtkType_t     ntkType;       // type of the network // ABC_NTK_STRASH, structurally hashed AIG (two input AND gates with c-attributes on edges)
        Abc_NtkFunc_t     ntkFunc;       // functionality of the network // ABC_FUNC_AIG,       // 3:  and-inverter graphs
        // char *            pName;         // the network name
        // char *            pSpec;         // the name of the spec file if present
        // Nm_Man_t *        pManName;      // name manager (stores names of objects)
        // components of the network
        Vec_Ptr_t *       vObjs;         // the array of all objects (net, nodes, latches, etc) // Vec<ptr>(100);
        Vec_Ptr_t *       vPis;          // the array of primary inputs // Vec<ptr>(100);
        Vec_Ptr_t *       vPos;          // the array of primary outputs // Vec<ptr>(100);
        Vec_Ptr_t *       vCis;          // the array of combinational inputs  (PIs, latches) // Vec<ptr>(100);
        Vec_Ptr_t *       vCos;          // the array of combinational outputs (POs, asserts, latches) // Vec<ptr>(100);
        Vec_Ptr_t *       vPios;         // the array of PIOs // Vec<ptr>(100);
        Vec_Ptr_t *       vBoxes;        // the array of boxes // Vec<ptr>(100);s
        Vec_Ptr_t *       vLtlProperties;
        // the number of living objects
        // int nObjCounts[ABC_OBJ_NUMBER];  // the number of objects by type
        // int               nObjs;         // the number of live objs
        // int               nConstrs;      // the number of constraints
        // int               nBarBufs;      // the number of barrier buffers
        // int               nBarBufs2;     // the number of barrier buffers
        // the backup network and the step number
        // Abc_Ntk_t *       pNetBackup;    // the pointer to the previous backup network
        // int               iStep;         // the generation number for the given network
        // hierarchy
        // Abc_Des_t *       pDesign;       // design (hierarchical networks only)
        // Abc_Ntk_t *       pAltView;      // alternative structural view of the network
        // int               fHieVisited;   // flag to mark the visited network
        // int               fHiePath;      // flag to mark the network on the path
        // int               Id;            // model ID
        // double            dTemp;         // temporary value
        // miscellaneous data members
        int               nTravIds;      // the unique traversal IDs of nodes // 1
        // Vec_Int_t         vTravIds;      // trav IDs of the objects
        Mem_Fixed_t *     pMmObj;        // memory manager for objects
        Mem_Step_t *      pMmStep;       // memory manager for arrays
        void *            pManFunc;      // functionality manager (AIG manager, BDD manager, or memory manager for SOPs) //
        Abc_ManTime_t *   pManTime;      // the timing manager (for mapped networks) stores arrival/required times for all nodes
        void *            pManCut;       // the cut manager (for AIGs) stores information about the cuts computed for the nodes
        float             AndGateDelay;  // an average estimated delay of one AND gate
        int               LevelMax;      // maximum number of levels
        Vec_Int_t *       vLevelsR;      // level in the reverse topological order (for AIGs)
        Vec_Ptr_t *       vSupps;        // CO support information
        int *             pModel;        // counter-example (for miters)
        Abc_Cex_t *       pSeqModel;     // counter-example (for sequential miters)
        Vec_Ptr_t *       vSeqModelVec;  // vector of counter-examples (for sequential miters)
        Abc_Ntk_t *       pExdc;         // the EXDC network (if given)
        void *            pExcare;       // the EXDC network (if given)
        void *            pData;         // misc
        Abc_Ntk_t *       pCopy;         // copy of this network
        void *            pBSMan;        // application manager
        void *            pSCLib;        // SC library
        Vec_Int_t *       vGates;        // SC library gates
        Vec_Int_t *       vPhases;       // fanins phases in the mapped netlist
        char *            pWLoadUsed;    // wire load model used
        float *           pLutTimes;     // arrivals/requireds/slacks using LUT-delay model
        Vec_Ptr_t *       vOnehots;      // names of one-hot-encoded registers
        Vec_Int_t *       vObjPerm;      // permutation saved
        Vec_Int_t *       vTopo;
        Vec_Ptr_t *       vAttrs;        // managers of various node attributes (node functionality, global BDDs, etc)
        Vec_Int_t *       vNameIds;      // name IDs
        Vec_Int_t *       vFins;         // obj/type info
    */
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl AndInverterGraph {
    // ************************************************************************************************
    // helper functions
    // ************************************************************************************************

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

    fn check_first_line_of_aig_and_load_it(&mut self, vector_of_lines_as_vectors: &Vec<Vec<u8>>) {
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

        self.maximum_variable_index = maximum_variable_index;
        self.number_of_inputs = number_of_inputs;
        self.number_of_latches = number_of_latches;
        self.number_of_outputs = number_of_outputs;
        self.number_of_and_gates = number_of_and_gates;
        self.number_of_bad_outputs = number_of_bad_outputs;
        self.number_of_constraint_outputs = number_of_constraint_outputs;
        self.number_of_justice_outputs = number_of_justice_outputs;
        self.number_of_fairness_outputs = number_of_fairness_outputs;
    }

    // ************************************************************************************************
    // api functions
    // ************************************************************************************************

    pub fn from_vector_of_bytes(vec_of_bytes: &Vec<u8>) -> AndInverterGraph {
        let lines =
            Self::split_vector_of_bytes_to_vector_of_vector_of_bytes_using_newlines(vec_of_bytes);
        let mut aig = AndInverterGraph::default();
        aig.check_first_line_of_aig_and_load_it(&lines);
        aig
    }

    pub fn from_aig_path(file_path: &str) -> AndInverterGraph {
        let file_as_vec_of_bytes =
            fs::read(file_path).expect(format!("Unable to read the file {file_path}").as_str());
        Self::from_vector_of_bytes(&file_as_vec_of_bytes)
    }
}
