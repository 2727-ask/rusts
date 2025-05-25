mod var;
mod data_types;
mod conditions;
mod loops;
mod match_me;
mod structs;


fn main() {
    println!("Hello, world!");
    var::run_var();
    data_types::data_types();
    conditions::conditions();
    loops::run_loop();
    match_me::match_me();
    structs::structs();
}