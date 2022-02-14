#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate sat_portfolio;

mod minisat {
    #![allow(unused)]
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
