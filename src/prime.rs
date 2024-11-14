use rand::Rng;

// Sieve of Eratosthenes Algorithm
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..=limit {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }

    is_prime.iter()
        .enumerate()
        .filter_map(|(num, &prime)| if prime { Some(num) } else { None })
        .collect()
}

// Random prime number generation
pub fn rand_prime(bound:i64) -> i64 {
    let mut rng = rand::thread_rng();
    loop {
        let value = rng.gen_range(2..=bound);
        if (sieve_of_eratosthenes(value as usize))
            .last()
            .map(|&last_prime| last_prime == value as usize)
            == Some(true) {
                return value
            }
    }
}