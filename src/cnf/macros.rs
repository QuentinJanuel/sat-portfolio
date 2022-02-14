/// A macro to create a clause
#[macro_export]
macro_rules! clause {
    ($( $x:expr ),*) => {
        use $crate::cnf::{
            Clause,
            Lit,
            Var,
        };
        Clause::from(
            vec![$($x),*]
                .iter()
                .map(|x: &i32| {
                    if *x > 0 {
                        Lit::pos(Var(*x as u32))
                    } else {
                        Lit::neg(Var(-*x as u32))
                    }
                })
                .collect()
        )
    };
}

/// A macro to create CNF formulas
#[macro_export]
macro_rules! cnf {
    () => {
        CNF::new()
    };
    ($( $( $x:expr ),*);*) => {
        {
            use $crate::cnf::{
                CNF,
                Clause,
                Lit,
                Var,
            };
            let data_as_nested_array = [ $( vec![ $($x),* ] ),* ];
            let mut cnf = CNF::new();
            for clause in data_as_nested_array.iter() {
                let clause = Clause::from(
                    clause
                        .iter()
                        .map(|x: &i32| {
                            if *x > 0 {
                                Lit::pos(Var(*x as u32))
                            } else {
                                Lit::neg(Var(-*x as u32))
                            }
                        })
                        .collect()
                );
                cnf.add_clause(clause);
            }
            cnf
        }
    };
}
