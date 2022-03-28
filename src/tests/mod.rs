use crate::{
    solver::{
        Solver,
        dpll::DPLL,
        minisat::Minisat,
        manysat::Manysat,
        glucose::Glucose,
    },
};

fn test_solver<S: Solver>(solver: S) {
    let cnf = cnf![
        1, 2;
        1
    ];
    let models = solver.get_all_models(&mut cnf.clone());
    assert_eq!(models.len(), 2);
    // let cnf = cnf![
    //     1;
    //     -1
    // ];
    // assert!(solver.solve(&cnf).is_none());
}

#[test]
fn test_solvers() {
    // test_solver(DPLL::new());
    // test_solver(Minisat::new());
    // test_solver(Manysat::new());
    // test_solver(portfolio![
    //     DPLL::new(),
    //     Minisat::new(),
    //     Manysat::new(),
    // ]);
    // test_solver(portfolio![Manysat::new()]);
    test_solver(Glucose::new());
    // test_solver({
    //     let mut s = Glucose::new();
    //     s.enable_preprocessing();
    //     s
    // });
    // test_solver(portfolio![
    //     Glucose::new(),
    //     DPLL::new(),
    // ]);
}
