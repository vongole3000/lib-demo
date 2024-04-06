mod generator;
pub fn print_random_nomber() {
    let n = generator::gen_ran();
    println!("Random u:8: {}", n);
}