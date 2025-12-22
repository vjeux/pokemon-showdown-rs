use pokemon_showdown::PRNG;

fn main() {
    println!("Pokemon Showdown Rust Simulator");

    // Test PRNG
    let mut prng = PRNG::new(None);
    println!("Generated seed: {:?}", prng.get_seed());

    for i in 0..5 {
        println!("Random {}: {}", i, prng.random_int(100));
    }
}
