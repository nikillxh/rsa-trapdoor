use std::io;

pub fn trapdoor(a: i64, b:i64) {
    // Generating crate::primes of bits each
    let bits: i64 = 64;
    let p: i64 = crate::prime::rand_prime(bits);
    let q: i64 = crate::prime::rand_prime(bits);

    // Multiply to get modulus (n)
    let n: i64 = p * q;

    // Euler's toint function
    let phi: i64 = (p-1) * (q-1);

    // Random Public exponent
    let e: i64 = 17;

    // Associated Private exponent
    let d: i64 = crate::calc::ext_euclid(e as i64, phi as i64).1;

    // Encrypting values individually
    let encrypted_1 = encrypt(a, e as i64, n);
    let encrypted_2 = encrypt(b, e as i64, n);

    // Performing homomorphic addition
    let encrypted_sum = (encrypted_1 * encrypted_2) % n;

    // Decrypting the results
    let decrypted_sum = decrypt(encrypted_sum, d, n);

    // Asserting
    assert_eq!(1, (e*d)%phi);

    println!("Encrypted Sum: {}", encrypted_sum);
    println!("Press Enter for Decrypted Sum");

    let mut enter = String::new();
    let _ = io::stdin().read_line(&mut enter);
    
    println!("Decrypted Sum: {}", decrypted_sum);
}

// Encrypting a plaintext using public key
fn encrypt(plaintxt: i64, e: i64, n: i64) -> i64 {
    crate::calc::mod_exp(plaintxt, e, n)
}

// Decrypting a ciphertext using private key
fn decrypt(ciphertxt: i64, d: i64, n: i64) -> i64 {
    crate::calc::mod_exp(ciphertxt, d, n)
}