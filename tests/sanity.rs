#[cfg(test)]
mod tests {
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::Variable;
    use rust_formal_verification::formulas::CNF;

    #[test]
    fn making_formulas_and_turning_them_into_strings() {
        let x = "x";

        let v1 = Variable::new(1);
        let v2 = Variable::new(2);
        let v3 = Variable::new(3);
        let v4 = Variable::new(4);
        let v5 = Variable::new(5);
        let v6 = Variable::new(6);

        assert!(v1.to_string() == format!("{x}1"));
        assert!(v2.to_string() == format!("{x}2"));
        assert!(v3.to_string() == format!("{x}3"));
        assert!(v4.to_string() == format!("{x}4"));
        assert!(v5.to_string() == format!("{x}5"));
        assert!(v6.to_string() == format!("{x}6"));

        let l1 = Literal::new(v1, false);
        let l2 = Literal::new(v2, true);
        let l3 = Literal::new(v3, false);
        let l4 = Literal::new(v4, true);
        let l5 = Literal::new(v5, false);
        let l6 = Literal::new(v6, true);

        assert!(l1.to_string() == format!("{x}1"));
        assert!(l2.to_string() == format!("!{x}2"));
        assert!(l3.to_string() == format!("{x}3"));
        assert!(l4.to_string() == format!("!{x}4"));
        assert!(l5.to_string() == format!("{x}5"));
        assert!(l6.to_string() == format!("!{x}6"));

        let mut c0 = Clause::new(vec![]);
        assert!(c0.get_highest_variable_number() == 0);
        c0.add_literal(&l1);
        assert!(c0.get_highest_variable_number() == 1);
        c0.add_literal(&l2);
        assert!(c0.get_highest_variable_number() == 2);
        c0.add_literal(&l3);
        assert!(c0.get_highest_variable_number() == 3);

        let mut c00 = Clause::new(vec![]);
        assert!(c00.get_highest_variable_number() == 0);
        c00.add_literal(&l2);
        assert!(c00.get_highest_variable_number() == 2);
        c00.add_literal(&l1);
        assert!(c00.get_highest_variable_number() == 2);
        c00.add_literal(&l3);
        assert!(c00.get_highest_variable_number() == 3);

        let c000 = Clause::new(vec![l2, l3, l1]);
        assert!(c000.get_highest_variable_number() == 3);

        let mut c1 = Clause::new(vec![]);
        assert!(c1.get_highest_variable_number() == 0);
        c1.add_literal(&l4);
        assert!(c1.get_highest_variable_number() == 4);
        c1.add_literal(&l5);
        assert!(c1.get_highest_variable_number() == 5);

        let mut c2 = Clause::new(vec![]);
        assert!(c2.get_highest_variable_number() == 0);
        c2.add_literal(&l1);
        assert!(c2.get_highest_variable_number() == 1);
        c2.add_literal(&l4);
        assert!(c2.get_highest_variable_number() == 4);
        c2.add_literal(&l5);
        assert!(c2.get_highest_variable_number() == 5);
        c2.add_literal(&l6);
        assert!(c2.get_highest_variable_number() == 6);

        assert!(c0.to_string() == format!("({x}1 | !{x}2 | {x}3)"));
        assert!(c00.to_string() == format!("({x}1 | !{x}2 | {x}3)"));
        assert!(c000.to_string() == format!("({x}1 | !{x}2 | {x}3)"));
        assert!(c1.to_string() == format!("(!{x}4 | {x}5)"));
        assert!(c2.to_string() == format!("({x}1 | !{x}4 | {x}5 | !{x}6)"));

        let mut cnf0 = CNF::default();
        assert!(cnf0.get_highest_variable_number() == 0);
        cnf0.add_clause(&(c0.clone()));
        assert!(cnf0.get_highest_variable_number() == 3);
        cnf0.add_clause(&(c1.clone()));
        assert!(cnf0.get_highest_variable_number() == 5);
        cnf0.add_clause(&(c2.clone()));
        assert!(cnf0.get_highest_variable_number() == 6);

        let mut cnf1 = CNF::default();
        assert!(cnf1.get_highest_variable_number() == 0);
        cnf1.add_clause(&(c0.clone()));
        assert!(cnf1.get_highest_variable_number() == 3);
        cnf1.add_clause(&(c1.clone()));
        assert!(cnf1.get_highest_variable_number() == 5);
        cnf1.add_clause(&(c1.clone()));
        assert!(cnf1.get_highest_variable_number() == 5);

        let mut cnf2 = CNF::default();
        assert!(cnf2.get_highest_variable_number() == 0);
        cnf2.add_clause(&(c0.clone()));
        assert!(cnf2.get_highest_variable_number() == 3);
        cnf2.add_clause(&(c00.clone()));
        assert!(cnf2.get_highest_variable_number() == 3);
        cnf2.add_clause(&(c000.clone()));
        assert!(cnf2.get_highest_variable_number() == 3);

        assert!(
            cnf0.to_string()
                == format!(
                    "((!{x}4 | {x}5) & ({x}1 | !{x}2 | {x}3) & ({x}1 | !{x}4 | {x}5 | !{x}6))"
                )
        );
        assert!(cnf1.to_string() == format!("((!{x}4 | {x}5) & ({x}1 | !{x}2 | {x}3))"));
        assert!(cnf2.to_string() == format!("(({x}1 | !{x}2 | {x}3))"));
    }

    #[test]
    fn making_formulas_and_sat_solving_them() {}
}
