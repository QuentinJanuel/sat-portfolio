#include <iostream>
#include "simp/SimpSolver.h"

struct maplesat_solver_t : public Maplesat::Solver {
    Maplesat::vec<Maplesat::Lit> clause;
};

extern "C" {

#include "maplesat.h"

maplesat_solver* maplesat_new() {
    return new maplesat_solver_t();
}

void maplesat_delete(maplesat_solver* s) {
    delete s;
}

void maplesat_add_clause_begin(maplesat_solver* s) {
    s->clause.clear();
}

void maplesat_add_clause_add_lit(maplesat_solver* s, maplesat_lit p) {
    s->clause.push(Maplesat::toLit(p));
}

maplesat_bool maplesat_add_clause_commit(maplesat_solver* s) {
    return s->addClause(s->clause);
}

maplesat_lit maplesat_new_lit(maplesat_solver *s) {
    return Maplesat::toInt(Maplesat::mkLit(s->newVar()));
}

maplesat_lit maplesat_make_lit(maplesat_var x) {
    return Maplesat::toInt(Maplesat::mkLit(x));
}

maplesat_lit maplesat_negate_lit(maplesat_lit p) {
    return Maplesat::toInt(~Maplesat::toLit(p));
}

maplesat_var maplesat_lit_to_var(maplesat_lit p) {
    return Maplesat::var(Maplesat::toLit(p));
}

maplesat_bool maplesat_solve(maplesat_solver* s) {
    return s->solve();
}

maplesat_lbool maplesat_model_value_var(maplesat_solver* s, maplesat_var x) {
    Maplesat::lbool b = s->modelValue(x);
    if (b == Maplesat::lbool((uint8_t)0))
        return maplesat_ltrue;
    else if (b == Maplesat::lbool((uint8_t)1))
        return maplesat_lfalse;
    return maplesat_lundef;
}

}
