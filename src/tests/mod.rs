use crate::{
    solver::{
        Solver,
        dpll::DPLL,
        minisat::Minisat,
        portfolio::Portfolio,
    },
    cnf,
};

fn test_solver<S: Solver>(solver: S) {
    let cnf = cnf![
        1,  2;
        1
    ];
    let models = solver.get_all_models(&cnf, None);
    assert_eq!(models.len(), 2);
    let cnf = cnf![
        1;
        -1
    ];
    assert!(DPLL::new().solve(&cnf).is_none());
}

#[test]
fn count_models() {
    test_solver(DPLL::new());
    test_solver(Minisat::new());
    let portfolio = Portfolio::from(vec![
        Box::new(DPLL::new()),
        Box::new(Minisat::new()),
    ]);
    test_solver(portfolio);
}
