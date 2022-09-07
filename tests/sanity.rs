#[cfg(test)]
mod tests {
    use rust_formal_verification::formulas::Variable;
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::CNF;

    #[test]
    fn making_formulas_and_turning_them_into_strings() {
        let x = "x";
        let v0 = Variable::new(0);
        assert!(v0.to_string() == format!("{x}0"));
        let v1 = Variable::new(1);
        assert!(v1.to_string() == format!("{x}1"));
        let v2 = Variable::new(2);
        assert!(v2.to_string() == format!("{x}2"));
        let v3 = Variable::new(3);
        assert!(v3.to_string() == format!("{x}3"));
        let v4 = Variable::new(4);
        assert!(v4.to_string() == format!("{x}4"));
        let v5 = Variable::new(5);
        assert!(v5.to_string() == format!("{x}5"));

        let l0 = Literal::new(v0, false);
        assert!(l0.to_string() == format!("{x}0"));
        let l1 = Literal::new(v1, true);
        assert!(l1.to_string() == format!("!{x}1"));
        let l2 = Literal::new(v2, false);
        assert!(l2.to_string() == format!("{x}2"));
        let l3 = Literal::new(v3, true);
        assert!(l3.to_string() == format!("!{x}3"));
        let l4 = Literal::new(v4, false);
        assert!(l4.to_string() == format!("{x}4"));
        let l5 = Literal::new(v5, true);
        assert!(l5.to_string() == format!("!{x}5"));

        let mut c0 = Clause::default();
        c0.add_literal(&l0);
        c0.add_literal(&l1);
        c0.add_literal(&l2);
        assert!(c0.to_string() == format!("({x}0 | !{x}1 | {x}2)"));
        // println!("c0 = {}", c0);

        let mut c1 = Clause::default();
        c1.add_literal(&l3);
        c1.add_literal(&l4);
        assert!(c1.to_string() == format!("(!{x}3 | {x}4)"));
        // println!("c1 = {}", c1);

        let mut c2 = Clause::default();
        c2.add_literal(&l0);
        c2.add_literal(&l3);
        c2.add_literal(&l4);
        c2.add_literal(&l5);
        assert!(c2.to_string() == format!("({x}0 | !{x}3 | {x}4 | !{x}5)"));
        // println!("c2 = {}", c2);

        let mut cnf0 = CNF::default();
        cnf0.add_clause(&(c0.clone()));
        cnf0.add_clause(&(c1.clone()));
        cnf0.add_clause(&(c2.clone()));
        println!("cnf0 = {}", cnf0);
        assert!(cnf0.to_string() == format!("(({x}0 | !{x}3 | {x}4 | !{x}5) & ({x}0 | !{x}1 | {x}2) & (!{x}3 | {x}4))"));

        let mut cnf1 = CNF::default();
        cnf1.add_clause(&(c0.clone()));
        cnf1.add_clause(&(c1.clone()));
        cnf1.add_clause(&(c1.clone()));
        assert!(cnf1.to_string() == format!("((!{x}3 | {x}4) & ({x}0 | !{x}1 | {x}2))"));
        // println!("cnf1 = {}", cnf1);

        let mut cnf2 = CNF::default();
        cnf2.add_clause(&(c0.clone()));
        cnf2.add_clause(&(c0.clone()));
        cnf2.add_clause(&(c0.clone()));
        assert!(cnf2.to_string() == format!("(({x}0 | !{x}1 | {x}2))"));
        // println!("cnf2 = {}", cnf2);
        
    }
}

