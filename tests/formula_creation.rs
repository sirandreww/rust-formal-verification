// ************************************************************************************************
// test mod declaration
// ************************************************************************************************

#[cfg(test)]
mod tests {

    // ********************************************************************************************
    // use
    // ********************************************************************************************

    use pretty_assertions::assert_eq;
    use rust_formal_verification::formulas::{Clause, Literal, CNF};

    // ********************************************************************************************
    // helper functions
    // ********************************************************************************************

    // ********************************************************************************************
    // test functions
    // ********************************************************************************************

    #[test]
    fn making_formulas_and_turning_them_into_strings() {
        let l1 = Literal::new(1);
        let l2 = !Literal::new(2);
        let l3 = Literal::new(3);
        let l4 = !Literal::new(4);
        let l5 = Literal::new(5);
        let l6 = !Literal::new(6);

        assert_eq!(l1.to_string(), ("1"));
        assert_eq!(l2.to_string(), ("-2"));
        assert_eq!(l3.to_string(), ("3"));
        assert_eq!(l4.to_string(), ("-4"));
        assert_eq!(l5.to_string(), ("5"));
        assert_eq!(l6.to_string(), ("-6"));

        let c0 = Clause::new(&[l1, l2, l3]);
        let c00 = Clause::new(&[l2, l1, l3]);
        let c000 = Clause::new(&[l2, l3, l1]);
        let c1 = Clause::new(&[l4, l5]);
        let c2 = Clause::new(&[l1, l4, l5, l6]);

        assert_eq!(c0.to_string(), ("1 -2 3 0"));
        assert_eq!(c00.to_string(), ("1 -2 3 0"));
        assert_eq!(c000.to_string(), ("1 -2 3 0"));
        assert_eq!(c1.to_string(), ("-4 5 0"));
        assert_eq!(c2.to_string(), ("1 -4 5 -6 0"));

        let mut cnf0 = CNF::new();
        cnf0.add_clause(&(c0.clone()));
        cnf0.add_clause(&(c1.clone()));
        cnf0.add_clause(&(c2.clone()));

        let mut cnf1 = CNF::new();
        cnf1.add_clause(&(c0.clone()));
        cnf1.add_clause(&(c1.clone()));
        cnf1.add_clause(&(c1.clone()));

        let mut cnf2 = CNF::new();
        cnf2.add_clause(&(c0.clone()));
        cnf2.add_clause(&(c00.clone()));
        cnf2.add_clause(&(c000.clone()));

        assert_eq!(
            cnf0.to_string(),
            ("p cnf 6 3\n-4 5 0\n1 -2 3 0\n1 -4 5 -6 0")
        );
        assert_eq!(cnf1.to_string(), ("p cnf 5 2\n-4 5 0\n1 -2 3 0"));
        assert_eq!(cnf2.to_string(), ("p cnf 3 1\n1 -2 3 0"));
    }

    #[test]
    fn double_negation() {
        let l1 = Literal::new(1);
        assert_eq!(l1.to_string(), ("1"));

        let l1_not = !l1;
        assert_eq!(l1_not.to_string(), ("-1"));

        let l1_not_not = !l1_not;
        assert_eq!(l1_not_not.to_string(), ("1"));
    }
}
