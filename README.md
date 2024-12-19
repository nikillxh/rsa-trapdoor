# Homomorphic Encryption with RSA in Rust

This project implements a basic homomorphic encryption scheme using RSA in Rust. It demonstrates the ability to perform computations (in this case, addition) directly on encrypted data, ensuring data privacy throughout the process.

## Features
- **Homomorphic Addition**: Add two encrypted numbers without decrypting them first.
- **RSA Encryption and Decryption**: Utilizes RSA encryption for secure data handling.
- **Random Prime Generation**: Randomly generates large primes using the Sieve of Eratosthenes algorithm.
- **Modular Arithmetic**: Implements modular exponentiation and extended Euclidean algorithms for RSA operations.

## File Structure
- **`calc.rs`**: Contains mathematical utilities, including the Extended Euclidean Algorithm and modular exponentiation.
- **`prime.rs`**: Implements the Sieve of Eratosthenes for prime generation and random prime number generation within a specified range.
- **`trapdoor.rs`**: Handles homomorphic encryption and decryption processes using RSA, performing the homomorphic addition operation.
- **`main.rs`**: The entry point of the program that allows the user to input two natural numbers and demonstrates homomorphic addition.

## Mechanism
1. **Prime Generation**: Two random large prime numbers `p` and `q` are generated using the `rand_prime` function from `prime.rs`.
2. **RSA Setup**: Using the generated primes, we compute `n = p * q` and the Euler’s totient function `φ(n)`. A public exponent `e` (set to 17) is chosen, and the private exponent `d` is computed using the extended Euclidean algorithm.
3. **Homomorphic Encryption**: The two numbers provided by the user are encrypted individually using RSA encryption. Their encrypted versions are then multiplied to achieve the encrypted sum (homomorphic property).
4. **Decryption**: The encrypted sum is decrypted using the private key `d` to reveal the result of the homomorphic addition.

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/nikillxh/rsa-trapdoor
    cd rsa-trapdoor
    ```

2. Build and run the project:
    ```bash
    cargo run
    ```

3. Input two natural numbers when prompted to perform homomorphic addition.
