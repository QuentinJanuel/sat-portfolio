use crate::{
    solver::{
        Solver,
        dpll::DPLL,
    },
    cnf,
};

#[test]
fn count_models() {
    let cnf = cnf![
        1,  2;
        1
    ];
    let models = DPLL::new()
        .get_all_models(cnf);
    assert_eq!(models.len(), 2);
    let cnf = cnf![
        1;
        -1
    ];
    assert!(DPLL::new().solve(cnf).is_none());
}
