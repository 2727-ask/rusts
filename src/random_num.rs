use rand::Rng;

pub fn rand_num() -> i32 {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_number);
    return random_number;
}