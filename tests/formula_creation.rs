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

        assert_eq!(l1.to_string(), format!("x1"));
        assert_eq!(l2.to_string(), format!("!x2"));
        assert_eq!(l3.to_string(), format!("x3"));
        assert_eq!(l4.to_string(), format!("!x4"));
        assert_eq!(l5.to_string(), format!("x5"));
        assert_eq!(l6.to_string(), format!("!x6"));

        let c0 = Clause::new(&[l1, l2, l3]);
        assert_eq!(c0.get_highest_variable_number(), 3);

        let c00 = Clause::new(&[l2, l1, l3]);
        assert_eq!(c00.get_highest_variable_number(), 3);

        let c000 = Clause::new(&[l2, l3, l1]);
        assert_eq!(c000.get_highest_variable_number(), 3);

        let c1 = Clause::new(&[l4, l5]);
        assert_eq!(c1.get_highest_variable_number(), 5);

        let c2 = Clause::new(&[l1, l4, l5, l6]);
        assert_eq!(c2.get_highest_variable_number(), 6);

        assert_eq!(c0.to_string(), format!("(x1 | !x2 | x3)"));
        assert_eq!(c00.to_string(), format!("(x1 | !x2 | x3)"));
        assert_eq!(c000.to_string(), format!("(x1 | !x2 | x3)"));
        assert_eq!(c1.to_string(), format!("(!x4 | x5)"));
        assert_eq!(c2.to_string(), format!("(x1 | !x4 | x5 | !x6)"));

        let mut cnf0 = CNF::new();
        assert_eq!(cnf0.get_highest_variable_number(), 0);
        cnf0.add_clause(&(c0.clone()));
        assert_eq!(cnf0.get_highest_variable_number(), 3);
        cnf0.add_clause(&(c1.clone()));
        assert_eq!(cnf0.get_highest_variable_number(), 5);
        cnf0.add_clause(&(c2.clone()));
        assert_eq!(cnf0.get_highest_variable_number(), 6);

        let mut cnf1 = CNF::new();
        assert_eq!(cnf1.get_highest_variable_number(), 0);
        cnf1.add_clause(&(c0.clone()));
        assert_eq!(cnf1.get_highest_variable_number(), 3);
        cnf1.add_clause(&(c1.clone()));
        assert_eq!(cnf1.get_highest_variable_number(), 5);
        cnf1.add_clause(&(c1.clone()));
        assert_eq!(cnf1.get_highest_variable_number(), 5);

        let mut cnf2 = CNF::new();
        assert_eq!(cnf2.get_highest_variable_number(), 0);
        cnf2.add_clause(&(c0.clone()));
        assert_eq!(cnf2.get_highest_variable_number(), 3);
        cnf2.add_clause(&(c00.clone()));
        assert_eq!(cnf2.get_highest_variable_number(), 3);
        cnf2.add_clause(&(c000.clone()));
        assert_eq!(cnf2.get_highest_variable_number(), 3);

        assert_eq!(
            cnf0.to_string(),
            format!("((!x4 | x5) & (x1 | !x2 | x3) & (x1 | !x4 | x5 | !x6))")
        );
        assert_eq!(cnf1.to_string(), format!("((!x4 | x5) & (x1 | !x2 | x3))"));
        assert_eq!(cnf2.to_string(), format!("((x1 | !x2 | x3))"));
    }

    #[test]
    fn double_negation() {
        let l1 = Literal::new(1);
        assert_eq!(l1.to_string(), format!("x1"));

        let l1_not = !l1;
        assert_eq!(l1_not.to_string(), format!("!x1"));

        let l1_not_not = !l1_not;
        assert_eq!(l1_not_not.to_string(), format!("x1"));
    }
}
