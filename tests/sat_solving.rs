// ************************************************************************************************
// mod declaration
// ************************************************************************************************

// mod common;

// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::CNF;
    use rust_formal_verification::solvers::sat::SatResponse;
    use rust_formal_verification::solvers::sat::SplrSolver;

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

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
}
