use rand::Rng;

pub fn run() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number: {secret_number}")
}
