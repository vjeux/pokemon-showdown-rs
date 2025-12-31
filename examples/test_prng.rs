/// Simple test to trace PRNG behavior

use pokemon_showdown::{PRNG, PRNGSeed};

fn main() {
    println!("=== Testing Rust PRNG ===");
    println!("Initial seed: [0, 0, 0, 1]");

    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));

    println!("\nFirst 10 random_range(0, 100) calls:");
    for i in 0..10 {
        let val = prng.random_range(0, 100);
        println!("{}: {}", i, val);
    }

    println!("\n=== Testing with fresh PRNG ===");
    let mut prng2 = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));
    println!("First random_range(50, 101): {}", prng2.random_range(50, 101));
    println!("Second random_range(50, 101): {}", prng2.random_range(50, 101));
}
