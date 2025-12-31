/// Detailed trace of EV generation to find the 1-call discrepancy

use pokemon_showdown::{PRNG, PRNGSeed};

fn main() {
    let mut prng = PRNG::new(Some(PRNGSeed::Gen5([0, 0, 0, 1])));

    // Skip to call 10 (after moves, before EVs)
    // Based on trace: species(1), level(2), item(3), nature(4), gender(5), moves(9)
    for _ in 0..9 {
        prng.random_range(0, 100);
    }

    let mut call_count = 9;

    println!("Starting EV generation at call {}\n", call_count + 1);

    // EV generation with detailed logging
    let mut evs = [0i32; 6];
    let mut total_evs = 0;
    let mut iteration = 0;

    while total_evs < 510 {
        iteration += 1;
        let calls_before = call_count;

        let mut available_stats = Vec::new();
        for (i, &ev) in evs.iter().enumerate() {
            if ev < 252 {
                available_stats.push(i);
            }
        }

        if available_stats.is_empty() {
            break;
        }

        call_count += 1;
        let stat_idx = *prng.sample(&available_stats).expect("No available stats");
        let call_after_sample = call_count;

        call_count += 1;
        let amount = prng.random_range(1, 5)
            .min(252 - evs[stat_idx])
            .min(510 - total_evs);
        let call_after_random = call_count;

        evs[stat_idx] += amount;
        total_evs += amount;

        println!(
            "[Iter {}] calls {}-{}: available={}, stat_idx={}, amount={}, total_evs={}, evs=[{},{},{},{},{},{}]",
            iteration,
            calls_before + 1,
            call_after_random,
            available_stats.len(),
            stat_idx,
            amount,
            total_evs,
            evs[0], evs[1], evs[2], evs[3], evs[4], evs[5]
        );
    }

    println!("\nEV generation complete: {} iterations, {} total calls", iteration, call_count - 9);
    println!("\nFinal EVs: hp:{} atk:{} def:{} spa:{} spd:{} spe:{}", evs[0], evs[1], evs[2], evs[3], evs[4], evs[5]);
    println!("Total calls after EVs: {}", call_count);
}
