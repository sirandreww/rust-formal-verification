// ************************************************************************************************
// mod declaration
// ************************************************************************************************

mod common;

// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use std::cmp::min;
    use std::time;
    use rand::Rng;
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::CNF;
    use rust_formal_verification::solvers::sat::SatResponse;
    use rust_formal_verification::solvers::sat::SplrSolver;
    use rust_formal_verification::solvers::sat::VarisatSolver;

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_cnf_on_sat_solvers(cnf: &CNF) -> (f32, f32) {
        // println!("test_cnf_on_sat_solvers - cnf = {}", cnf);

        let splr_timer = time::Instant::now();
        let splr_response = SplrSolver::default().solve_cnf(&cnf);
        let splr_time = splr_timer.elapsed().as_secs_f32();

        let varisat_timer = time::Instant::now();
        let varisat_response = VarisatSolver::default().solve_cnf(&cnf);
        let varisat_time = varisat_timer.elapsed().as_secs_f32();
        
        match (splr_response, varisat_response) {
            (SatResponse::Sat { assignment:_ }, SatResponse::Sat { assignment:_ }) => {},
            (SatResponse::UnSat, SatResponse::UnSat) => {},
            _ => panic!("Sat solvers disagree."),
        }
        
        // println!("splr_time = {}, varisat_time = {}", splr_time, varisat_time);
        (splr_time, varisat_time)
    }

    // ********************************************************************************************
    // test functions
    // ********************************************************************************************

    #[test]
    fn sat_solve_simple_cnf() {
        let mut cnf = CNF::new();

        let l1 = Literal::new(1);
        let l2 = Literal::new(2);
        let l3 = Literal::new(3);

        cnf.add_clause(&Clause::new(&[l1, l2, l3]));
        cnf.add_clause(&Clause::new(&[!l1, l2, l3]));
        cnf.add_clause(&Clause::new(&[l1, !l2, l3]));
        cnf.add_clause(&Clause::new(&[l1, l2, !l3]));

        let solver = SplrSolver::default();
        let response = solver.solve_cnf(&cnf);
        match response {
            SatResponse::Sat { assignment: _ } => assert!(true),
            SatResponse::UnSat => assert!(false),
        };
    }

    #[test]
    fn sat_solve_simple_cnf_2() {
        let mut cnf = CNF::new();

        let l1 = Literal::new(1);
        let l2 = Literal::new(2);
        let l3 = Literal::new(3);
        let l4 = Literal::new(4);
        let l5 = Literal::new(5);

        cnf.add_clause(&Clause::new(&[l4, !l5]));
        cnf.add_clause(&Clause::new(&[!l3]));
        cnf.add_clause(&Clause::new(&[!l3, !l4]));
        cnf.add_clause(&Clause::new(&[l1, !l4, l5]));
        cnf.add_clause(&Clause::new(&[!l1]));
        cnf.add_clause(&Clause::new(&[!l2, !l4]));
        cnf.add_clause(&Clause::new(&[l2, l3, l4]));
        cnf.add_clause(&Clause::new(&[!l2]));
        cnf.add_clause(&Clause::new(&[!l1, !l5]));
        cnf.add_clause(&Clause::new(&[l3]));

        let solver = SplrSolver::default();
        let response = solver.solve_cnf(&cnf);
        match response {
            SatResponse::Sat { assignment: _ } => assert!(false),
            SatResponse::UnSat => assert!(true),
        };
    }

    #[test]
    fn exhaustive_sat_test() {
        // in release mode this can be high
        // but otherwise it should be around 15. 
        let max_number_of_variables_in_cnf = 100;
        let mut varisat_total_time = 0_f32;
        let mut splr_total_time = 0_f32;

        for number_of_variables_in_cnf in 0..max_number_of_variables_in_cnf {
            println!("number_of_variables_in_cnf = {}", number_of_variables_in_cnf);
            // create the literals, notice that there numbers are not randomized, 
            // this is because most sat solvers assume that variable numbers should 
            // be very closely related to the number of variables in the CNF. This
            // convention is also followed throughout most of the library (We start with
            // the variable with the number 1, and add one each time we want a new variable)
            let mut literals = Vec::new();
            for i in 1..(1 + number_of_variables_in_cnf){
                literals.push(Literal::new(i));
            }

            // no more than ~2000 clauses per number_of_variables_in_cnf
            let max_number_of_clauses = 2_i32.pow(min(number_of_variables_in_cnf, 11));

            // go over number of clauses.
            for number_of_clauses in 0..max_number_of_clauses{
                let mut cnf = CNF::new();
                
                // length of each clause should not be so big as generally in FiniteStateTransition
                // one would have at most 3 literals per clause.
                // use geometric 
                for _ in 0..number_of_clauses{
                    let mut clause_literals = Vec::new();

                    // allow empty clause for small problems.
                    if number_of_variables_in_cnf > 2 {
                        // at least one literal so as to not make the problem trivial.
                        let lit = literals[rand::thread_rng().gen_range(0..literals.len())];
                        clause_literals.push(lit);
                    }
                    
                    // add more with geometric distribution
                    while common::_true_with_probability(0.5) {
                        let lit = literals[rand::thread_rng().gen_range(0..literals.len())];
                        clause_literals.push(lit);
                    }

                    cnf.add_clause(&Clause::new(&clause_literals));

                }

                let (splr, varisat) = self::test_cnf_on_sat_solvers(&cnf);
                varisat_total_time += varisat;
                splr_total_time += splr;
            }
        }

        println!("splr_total_time {}, varisat_total_time = {}", splr_total_time, varisat_total_time);
    }
}
