use crate::{
    dpll::DPLL,
    solver::Solver,
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
}
