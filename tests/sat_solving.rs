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
    use rust_formal_verification::algorithms::formula_logic::evaluate_assignment_on_cnf;
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::CNF;
    use rust_formal_verification::solvers::sat::stateful::MiniSatSolver;
    use rust_formal_verification::solvers::sat::stateless::CadicalSolver;
    use rust_formal_verification::solvers::sat::stateless::SplrSolver;
    use rust_formal_verification::solvers::sat::stateless::VarisatSolver;
    use rust_formal_verification::solvers::sat::SatResponse;
    use std::cmp::min;
    use std::collections::HashMap;
    use std::time;

    use crate::common;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    fn test_cnf_on_sat_solvers(cnf: &CNF, stats: &mut HashMap<&str, f32>) -> bool {
        // println!("test_cnf_on_sat_solvers - cnf = {}", cnf);
        let splr_timer = time::Instant::now();
        let splr_response = SplrSolver::default().solve_cnf(cnf);
        *stats.get_mut("SplrSolver").unwrap() += splr_timer.elapsed().as_secs_f32();

        let varisat_timer = time::Instant::now();
        let varisat_response = VarisatSolver::default().solve_cnf(cnf);
        *stats.get_mut("VarisatSolver").unwrap() += varisat_timer.elapsed().as_secs_f32();

        let cadical_timer = time::Instant::now();
        let cadical_response = CadicalSolver::default().solve_cnf(cnf);
        *stats.get_mut("CadicalSolver").unwrap() += cadical_timer.elapsed().as_secs_f32();

        let mini_sat_timer = time::Instant::now();
        let mut solver = MiniSatSolver::default();
        solver.add_cnf(&cnf);
        let mini_sat_response = solver.solve();
        *stats.get_mut("MiniSatSolver").unwrap() += mini_sat_timer.elapsed().as_secs_f32();

        // make sure all results are the same

        match (
            splr_response,
            varisat_response,
            cadical_response,
            mini_sat_response,
        ) {
            (
                SatResponse::Sat { assignment: a },
                SatResponse::Sat { assignment: b },
                SatResponse::Sat { assignment: c },
                SatResponse::Sat { assignment: d },
            ) => {
                assert!(evaluate_assignment_on_cnf(cnf, &a));
                assert!(evaluate_assignment_on_cnf(cnf, &b));
                assert!(evaluate_assignment_on_cnf(cnf, &c));
                assert!(evaluate_assignment_on_cnf(cnf, &d));
                true
            }
            (SatResponse::UnSat, SatResponse::UnSat, SatResponse::UnSat, SatResponse::UnSat) => {
                false
            }
            _ => panic!("Sat solvers disagree."),
        }
    }

    fn print_results(
        vec_of_number_of_sat: &[i64],
        vec_of_number_of_unsat: &[i64],
        stats: &HashMap<&str, f32>,
    ) {
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
            "total sat = {}, total unsat = {}",
            vec_of_number_of_sat
                .iter()
                .map(|x| x.to_owned())
                .sum::<i64>(),
            vec_of_number_of_unsat.iter().sum::<i64>()
        );
        println!("Stats:");
        for (key, value) in stats {
            println!("{}: {}", key, value);
        }
    }

    fn generate_random_cnf(
        number_of_clauses: i32,
        number_of_variables_in_cnf: u32,
        maximal_number_of_variables_in_cnf_to_allow_empty_clause: u32,
        probability_of_adding_another_literal_to_cnf: f64,
        probability_of_negation: f64,
    ) -> CNF {
        // create the literals, notice that there numbers are not randomized,
        // this is because most sat solvers assume that variable numbers should
        // be very closely related to the number of variables in the CNF. This
        // convention is also followed throughout most of the library (We start with
        // the variable with the number 1, and add one each time we want a new variable)
        let mut literals = Vec::new();
        for i in 1..(1 + number_of_variables_in_cnf) {
            literals.push(Literal::new(i));
        }

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
                    .negate_if_true(common::_true_with_probability(probability_of_negation));
                clause_literals.push(lit);
            }

            cnf.add_clause(&Clause::new(&clause_literals));
        }

        cnf
    }

    fn generate_cnf_and_call_it(
        number_of_clauses: i32,
        number_of_variables_in_cnf: u32,
        maximal_number_of_variables_in_cnf_to_allow_empty_clause: u32,
        probability_of_adding_another_literal_to_cnf: f64,
        probability_of_negation: f64,
        number_of_repeats: u32,
        stats: &mut HashMap<&str, f32>,
        number_of_sat: &mut i64,
        number_of_unsat: &mut i64,
    ) {
        let cnf = generate_random_cnf(
            number_of_clauses,
            number_of_variables_in_cnf,
            maximal_number_of_variables_in_cnf_to_allow_empty_clause,
            probability_of_adding_another_literal_to_cnf,
            probability_of_negation,
        );

        let is_sat = self::test_cnf_on_sat_solvers(&cnf, stats);
        for _ in 1..number_of_repeats {
            let new_result = self::test_cnf_on_sat_solvers(&cnf, stats);
            assert_eq!(
                is_sat, new_result,
                "Multiple calls to sat solvers with same cnf got different answers."
            )
        }
        if is_sat {
            *number_of_sat += 1;
        } else {
            *number_of_unsat += 1;
        }
    }

    // ********************************************************************************************
    // test functions
    // ********************************************************************************************

    #[test]
    fn random_sat_queries_performance_test() {
        let mut stats = HashMap::from([
            ("SplrSolver", (0_f32)),
            ("VarisatSolver", (0_f32)),
            ("CadicalSolver", (0_f32)),
            ("MiniSatSolver", (0_f32)),
        ]);
        let mut vec_of_number_of_sat = Vec::new();
        let mut vec_of_number_of_unsat = Vec::new();

        // play with this until number of sat and un-sat are close

        // in release mode this can be high
        // but otherwise it should be around 8.
        let max_number_of_variables_in_cnf = 10;
        let maximal_number_of_variables_in_cnf_to_allow_empty_clause = 2;
        let probability_of_negation = 0.002;
        let probability_of_adding_another_literal_to_cnf = 0.5;
        let base = 2_i32;
        let max_exponent = 11;
        let number_of_repeats = 3;

        for number_of_variables_in_cnf in 0..max_number_of_variables_in_cnf {
            let mut number_of_sat = 0;
            let mut number_of_unsat = 0;
            println!(
                "number_of_variables_in_cnf = {}",
                number_of_variables_in_cnf
            );

            // no more than ~2000 clauses per number_of_variables_in_cnf
            let max_number_of_clauses = base.pow(min(number_of_variables_in_cnf, max_exponent));

            // go over number of clauses.
            for number_of_clauses in 0..max_number_of_clauses {
                generate_cnf_and_call_it(
                    number_of_clauses,
                    number_of_variables_in_cnf,
                    maximal_number_of_variables_in_cnf_to_allow_empty_clause,
                    probability_of_adding_another_literal_to_cnf,
                    probability_of_negation,
                    number_of_repeats,
                    &mut stats,
                    &mut number_of_sat,
                    &mut number_of_unsat,
                );
            }
            vec_of_number_of_sat.push(number_of_sat);
            vec_of_number_of_unsat.push(number_of_unsat);
        }
        print_results(&vec_of_number_of_sat, &vec_of_number_of_unsat, &stats);
    }

    // ********************************************************************************************
    // trivial test functions
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
}
