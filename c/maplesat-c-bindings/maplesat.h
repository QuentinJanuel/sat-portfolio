#ifndef __BINDINGS__
#define __BINDINGS__

typedef struct maplesat_solver_t maplesat_solver;
typedef int maplesat_bool;
typedef int maplesat_lbool;
typedef int maplesat_var;
typedef int maplesat_lit;

extern const maplesat_lbool maplesat_ltrue = 1;
extern const maplesat_lbool maplesat_lfalse = 0;
extern const maplesat_lbool maplesat_lundef = -1;

maplesat_solver* maplesat_new();
void maplesat_delete(maplesat_solver* s);
void maplesat_add_clause_begin(maplesat_solver* s);
void maplesat_add_clause_add_lit(maplesat_solver* s, maplesat_lit p);
maplesat_bool maplesat_add_clause_commit(maplesat_solver* s);
maplesat_lit maplesat_new_lit(maplesat_solver* s);
maplesat_lit maplesat_make_lit(maplesat_var x);
maplesat_lit maplesat_negate_lit(maplesat_lit p);
maplesat_var maplesat_lit_to_var(maplesat_lit p);
maplesat_bool maplesat_solve(maplesat_solver* s);
maplesat_lbool maplesat_model_value_var(maplesat_solver* s, maplesat_var x);

#endif
