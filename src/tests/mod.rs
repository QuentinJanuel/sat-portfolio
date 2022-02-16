use crate::{
    solver::{
        Solver,
        dpll::DPLL,
        minisat::Minisat,
    },
};

fn test_solver<S: Solver>(solver: S) {
    let cnf = cnf![
        1, 2;
        1
    ];
    let models = solver.get_all_models(&cnf);
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
    test_solver(portfolio![
        DPLL::new(),
        Minisat::new(),
    ]);
}
