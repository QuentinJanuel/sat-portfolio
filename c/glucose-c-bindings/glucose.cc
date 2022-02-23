#include <iostream>
#include <pthread.h>
#include "simp/SimpSolver.h"

struct glucose_solver_t : public Glucose::SimpSolver {
    Glucose::vec<Glucose::Lit> clause;
};

extern "C" {

#include "glucose.h"

glucose_solver* glucose_new() {
    glucose_solver* s = new glucose_solver_t();
    s->simplify();
    return s;
}

void glucose_delete(glucose_solver* s) {
    delete s;
}

void glucose_add_clause_begin(glucose_solver* s) {
    s->clause.clear();
}

void glucose_add_clause_add_lit(glucose_solver* s, glucose_lit p) {
    s->clause.push(Glucose::toLit(p));
}

glucose_bool glucose_add_clause_commit(glucose_solver* s) {
    return s->addClause_(s->clause);
}

glucose_lit glucose_new_lit(glucose_solver* s) {
    return Glucose::toInt(Glucose::mkLit(s->newVar()));
}

glucose_lit glucose_make_lit(glucose_var x) {
    return Glucose::toInt(Glucose::mkLit(x));
}

glucose_lit glucose_negate_lit(glucose_lit p) {
    return Glucose::toInt(~Glucose::toLit(p));
}

glucose_var glucose_lit_to_var(glucose_lit p) {
    return Glucose::var(Glucose::toLit(p));
}

glucose_bool glucose_solve(glucose_solver* s) {
    if (!s->okay())
        return 0;
    return s->solve();
}

glucose_lbool glucose_model_value_var(glucose_solver* s, glucose_var x) {
    Glucose::lbool b = s->modelValue(x);
    if (b == Glucose::lbool((uint8_t)0))
        return glucose_ltrue;
    else if (b == Glucose::lbool((uint8_t)1))
        return glucose_lfalse;
    return glucose_lundef;
}

}
