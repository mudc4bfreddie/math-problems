
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..=100);

    println!("The number is {}", random_number);
}