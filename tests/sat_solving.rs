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

    use rand::Rng;
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::CNF;
    use rust_formal_verification::solvers::sat::SatResponse;
    use rust_formal_verification::solvers::sat::SplrSolver;
    use rust_formal_verification::solvers::sat::VarisatSolver;
    use rust_formal_verification::solvers::sat::CadicalSolver;
    use std::cmp::min;
    use std::time;

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_cnf_on_sat_solvers(cnf: &CNF) -> (f32, f32, f32, bool) {
        // println!("test_cnf_on_sat_solvers - cnf = {}", cnf);

        let splr_timer = time::Instant::now();
        let splr_response = SplrSolver::default().solve_cnf(&cnf);
        let splr_time = splr_timer.elapsed().as_secs_f32();

        let varisat_timer = time::Instant::now();
        let varisat_response = VarisatSolver::default().solve_cnf(&cnf);
        let varisat_time = varisat_timer.elapsed().as_secs_f32();

        let cadical_timer = time::Instant::now();
        let cadical_response = CadicalSolver::default().solve_cnf(&cnf);
        let cadical_time = cadical_timer.elapsed().as_secs_f32();

        match (splr_response, varisat_response, cadical_response) {
            (SatResponse::Sat { assignment: _ }, SatResponse::Sat { assignment: _ }, SatResponse::Sat { assignment: _ }) => {
                (splr_time, varisat_time, cadical_time, true)
            }
            (SatResponse::UnSat, SatResponse::UnSat, SatResponse::UnSat) => (splr_time, varisat_time, cadical_time, false),
            _ => panic!("Sat solvers disagree."),
        }
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
        // but otherwise it should be around 8.
        let max_number_of_variables_in_cnf = 10;
        let mut varisat_total_time = 0_f32;
        let mut splr_total_time = 0_f32;
        let mut cadical_total_time = 0_f32;
        let mut vec_of_number_of_sat = Vec::new();
        let mut vec_of_number_of_unsat = Vec::new();

        // play with this until number of sat and un-sat are close
        let maximal_number_of_variables_in_cnf_to_allow_empty_clause = 2;
        let probability_of_negation = 0.002;
        let probability_of_adding_another_literal_to_cnf = 0.5;
        let base = 2_i32;
        let max_exponent = 11;

        for number_of_variables_in_cnf in 0..max_number_of_variables_in_cnf {
            let mut number_of_sat = 0;
            let mut number_of_unsat = 0;
            println!(
                "number_of_variables_in_cnf = {}",
                number_of_variables_in_cnf
            );
            // create the literals, notice that there numbers are not randomized,
            // this is because most sat solvers assume that variable numbers should
            // be very closely related to the number of variables in the CNF. This
            // convention is also followed throughout most of the library (We start with
            // the variable with the number 1, and add one each time we want a new variable)
            let mut literals = Vec::new();
            for i in 1..(1 + number_of_variables_in_cnf) {
                literals.push(Literal::new(i));
            }

            // no more than ~2000 clauses per number_of_variables_in_cnf
            let max_number_of_clauses = base.pow(min(number_of_variables_in_cnf, max_exponent));

            // go over number of clauses.
            for number_of_clauses in 0..max_number_of_clauses {
                let mut cnf = CNF::new();

                // length of each clause should not be so big as generally in FiniteStateTransition
                // one would have at most 3 literals per clause.
                // use geometric
                for _ in 0..number_of_clauses {
                    let mut clause_literals = Vec::new();

                    // add more with geometric distribution
                    while (
                        (number_of_variables_in_cnf > maximal_number_of_variables_in_cnf_to_allow_empty_clause)
                        && (clause_literals.is_empty())) // if is empty and we cannot allow empty clause
                        || (common::_true_with_probability( // or we just want another variable
                            probability_of_adding_another_literal_to_cnf,
                        ))
                    {
                        let lit = literals[rand::thread_rng().gen_range(0..literals.len())]
                            .negate_if_true(common::_true_with_probability(
                                probability_of_negation,
                            ));
                        clause_literals.push(lit);
                    }

                    cnf.add_clause(&Clause::new(&clause_literals));
                }

                let (splr, varisat, cadical, is_sat) = self::test_cnf_on_sat_solvers(&cnf);
                if is_sat {
                    number_of_sat += 1;
                } else {
                    number_of_unsat += 1;
                }
                varisat_total_time += varisat;
                splr_total_time += splr;
                cadical_total_time += cadical;
            }
            vec_of_number_of_sat.push(number_of_sat);
            vec_of_number_of_unsat.push(number_of_unsat);
        }
        assert_eq!(vec_of_number_of_sat.len(), vec_of_number_of_unsat.len());
        for (i, (s, us)) in vec_of_number_of_sat
            .iter()
            .zip(vec_of_number_of_unsat.iter())
            .enumerate()
        {
            println!(
                "number_of_variables_in_cnf = {}, # sat = {}, #unsat = {}",
                i, s, us
            );
        }
        println!(
            "splr_total_time {}, varisat_total_time = {}, cadical_total_time = {}",
            splr_total_time, varisat_total_time, cadical_total_time
        );
        println!(
            "total sat = {}, total unsat = {}",
            vec_of_number_of_sat.iter().sum::<i64>(),
            vec_of_number_of_unsat.iter().sum::<i64>()
        );
    }
}
