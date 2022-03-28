#ifndef __BINDINGS__
#define __BINDINGS__

typedef struct glucose_solver_t glucose_solver;
typedef int glucose_bool;
typedef int glucose_lbool;
typedef int glucose_var;
typedef int glucose_lit;

extern const glucose_lbool glucose_ltrue = 1;
extern const glucose_lbool glucose_lfalse = 0;
extern const glucose_lbool glucose_lundef = -1;

glucose_solver* glucose_new(glucose_bool preprocessing, glucose_bool parallel);
void glucose_delete(glucose_solver* s);
void glucose_add_clause_begin(glucose_solver* s);
void glucose_add_clause_add_lit(glucose_solver* s, glucose_lit p);
glucose_bool glucose_add_clause_commit(glucose_solver* s);
glucose_lit glucose_new_lit(glucose_solver* s);
glucose_lit glucose_make_lit(glucose_var x);
glucose_lit glucose_negate_lit(glucose_lit p);
glucose_var glucose_lit_to_var(glucose_lit p);
glucose_bool glucose_solve(glucose_solver* s);
glucose_lbool glucose_model_value_var(glucose_solver* s, glucose_var x);

#endif
