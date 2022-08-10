use prime_sieves::pritchard::sieve_pritchard;

fn main() {
    let vec = sieve_pritchard(20000000);
    println!("{}", vec.len());
}
