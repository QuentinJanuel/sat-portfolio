#include "core/Solver.h"
#include "core/Cooperation.h"

struct manysat_solver_t : public Manysat::Solver {
    Manysat::vec<Manysat::Lit> clause;
};

extern "C" {

#include "manysat.h"

manysat_solver* manysat_new() {
    return new manysat_solver_t();
}

void manysat_delete(manysat_solver* s) {
    delete s;
}

void manysat_add_clause_begin(manysat_solver* s) {
    s->clause.clear();
}

void manysat_add_clause_add_lit(manysat_solver* s, manysat_lit p) {
    s->clause.push(Manysat::toLit(p));
}

manysat_bool manysat_add_clause_commit(manysat_solver* s) {
    return s->addClause(s->clause);
}

manysat_lit manysat_new_lit(manysat_solver *s) {
    return Manysat::toInt(Manysat::mkLit(s->newVar()));
}

manysat_lit manysat_make_lit(manysat_var x) {
    return Manysat::toInt(Manysat::mkLit(x));
}

manysat_lit manysat_negate_lit(manysat_lit p) {
    return Manysat::toInt(~Manysat::toLit(p));
}

manysat_var manysat_lit_to_var(manysat_lit p) {
    return Manysat::var(Manysat::toLit(p));
}

manysat_bool manysat_solve(manysat_solver* s, int n_threads, int clause_limit) {
    return s->solve(new Manysat::Cooperation(n_threads, clause_limit));
}

manysat_lbool manysat_model_value_var(manysat_solver* s, manysat_var x) {
    Manysat::lbool b = s->modelValue(x);
    if (b == Manysat::lbool((uint8_t)0))
        return manysat_ltrue;
    else if (b == Manysat::lbool((uint8_t)1))
        return manysat_lfalse;
    return manysat_lundef;
}

}
