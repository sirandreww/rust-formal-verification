#[cfg(test)]
mod tests {
    use rust_formal_verification::formulas::Literal;
    use rust_formal_verification::formulas::Clause;
    use rust_formal_verification::formulas::CNF;

    #[test]
    fn make_literal() {
        let l0 = Literal::new(0, false);
        println!("l0 = {}", l0);
        let l1 = Literal::new(1, true);
        println!("l1 = {}", l1);
        let l2 = Literal::new(2, true);
        println!("l2 = {}", l2);
        let l3 = Literal::new(3, true);
        println!("l3 = {}", l3);
        let l4 = Literal::new(4, true);
        println!("l4 = {}", l4);
        let l5 = Literal::new(5, true);
        println!("l5 = {}", l5);

        let mut c0 = Clause::default();
        c0.add_literal(&l0);
        c0.add_literal(&l1);
        c0.add_literal(&l2);
        println!("c0 = {}", c0);

        let mut c1 = Clause::default();
        c1.add_literal(&l3);
        c1.add_literal(&l4);
        println!("c1 = {}", c1);

        let mut c2 = Clause::default();
        c2.add_literal(&l0);
        c2.add_literal(&l3);
        c2.add_literal(&l4);
        c2.add_literal(&l5);
        println!("c2 = {}", c2);

        let mut cnf0 = CNF::default();
        cnf0.add_clause(&c0);
        cnf0.add_clause(&c1);
        cnf0.add_clause(&c2);
        println!("cnf0 = {}", cnf0);

        let mut cnf1 = CNF::default();
        cnf1.add_clause(&c0);
        cnf1.add_clause(&c1);
        cnf1.add_clause(&c1);
        println!("cnf1 = {}", cnf1);

        let mut cnf2 = CNF::default();
        cnf2.add_clause(&c0);
        cnf2.add_clause(&c0);
        cnf2.add_clause(&c0);
        println!("cnf2 = {}", cnf2);
        
    }
}

