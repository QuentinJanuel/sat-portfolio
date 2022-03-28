#include <iostream>
#include <pthread.h>
#include "simp/SimpSolver.h"
#include "parallel/MultiSolvers.h"

struct glucose_solver_t {
    Glucose::SimpSolver *simpsolver;
    Glucose::MultiSolvers *multisolver;
    bool parallel;
    bool preprocessing;
    Glucose::vec<Glucose::Lit> clause;
};

extern "C" {

#include "glucose.h"

glucose_solver* glucose_new(glucose_bool preprocessing, glucose_bool parallel) {
    glucose_solver* s = new glucose_solver_t();
    s->preprocessing = preprocessing;
    s->parallel = parallel;
    if (s->parallel) {
        s->multisolver = new Glucose::MultiSolvers();
    } else {
        s->simpsolver = new Glucose::SimpSolver();
        if (!s->preprocessing) {
            s->simpsolver->eliminate(true);
        }
    }
    return s;
}

void glucose_delete(glucose_solver* s) {
    if (s->parallel) {
        delete s->multisolver;
    } else {
        delete s->simpsolver;
    }
    delete s;
}

void glucose_add_clause_begin(glucose_solver* s) {
    s->clause.clear();
}

void glucose_add_clause_add_lit(glucose_solver* s, glucose_lit p) {
    s->clause.push(Glucose::toLit(p));
}

glucose_bool glucose_add_clause_commit(glucose_solver* s) {
    if (s->parallel) {
        return s->multisolver->addClause(s->clause);
    } else {
        return s->simpsolver->addClause(s->clause);
    }
}

glucose_lit glucose_new_lit(glucose_solver* s) {
    Glucose::Var v;
    if (s->parallel) {
        v = s->multisolver->newVar();
    } else {
        v = s->simpsolver->newVar();
    }
    return Glucose::toInt(Glucose::mkLit(v));
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
    if (s->parallel) {
        if (!s->multisolver->okay())
            return 0;
        return s->multisolver->solve() == l_True;
    } else {
        if (s->preprocessing) {
            s->simpsolver->eliminate(true);
        }
        if (!s->simpsolver->okay())
            return 0;
        return s->simpsolver->solve();
    }
}

glucose_lbool glucose_model_value_var(glucose_solver* s, glucose_var x) {
    Glucose::lbool b;
    if (s->parallel) {
        b = s->multisolver->model[x - 1];
    } else {
        b = s->simpsolver->modelValue(x);
    }
    if (b == l_True)
        return glucose_ltrue;
    else if (b == l_False)
        return glucose_lfalse;
    return glucose_lundef;
}

}
