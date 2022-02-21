#ifndef __BINDINGS__
#define __BINDINGS__

typedef struct manysat_solver_t manysat_solver;
typedef int manysat_bool;
typedef int manysat_lbool;
typedef int manysat_var;
typedef int manysat_lit;

extern const manysat_lbool manysat_ltrue = 1;
extern const manysat_lbool manysat_lfalse = 0;
extern const manysat_lbool manysat_lundef = -1;

manysat_solver* manysat_new();
void manysat_delete(manysat_solver* s);
void manysat_add_clause_begin(manysat_solver* s);
void manysat_add_clause_add_lit(manysat_solver* s, manysat_lit p);
manysat_bool manysat_add_clause_commit(manysat_solver* s);
manysat_lit manysat_new_lit(manysat_solver* s);
manysat_lit manysat_make_lit(manysat_var x);
manysat_lit manysat_negate_lit(manysat_lit p);
manysat_var manysat_lit_to_var(manysat_lit p);
manysat_bool manysat_solve(manysat_solver* s, int n_threads, int clause_limit);
manysat_lbool manysat_model_value_var(manysat_solver* s, manysat_var x);

#endif
