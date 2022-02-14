extern crate sat_portfolio;

mod minisat {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
    )]
    include!(concat!(env!("OUT_DIR"), "/minisat-bindings.rs"));
}

fn main() {
    unsafe {
        let ltrue = minisat::minisat_l_True;
        let lfalse = minisat::minisat_l_False;
        let lundef = minisat::minisat_l_Undef;
        println!("{}", ltrue);
        println!("{}", lfalse);
        println!("{}", lundef);
    }
    println!("Hello, world!");
}
