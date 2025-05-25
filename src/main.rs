mod var;
mod data_types;
mod conditions;
mod loops;
mod match_me;
mod structs;
mod random_num;


fn main() {
    println!("Hello, world!");
    var::run_var();
    data_types::data_types();
    conditions::conditions();
    loops::run_loop();
    match_me::match_me();
    structs::structs();
    let random_no = random_num::rand_num();
    println!("Random number generated: {:?}", random_no);
}