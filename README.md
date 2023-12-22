# Secure_Hash_Function

## Description
This project repository contains an implementation of a secure hash function in Rust that uses the SHA-256 algorithm. The SHA-256 algorithm is a widely used cryptographic hash function that is known for its collision resistance and avalanche effect.

## Implementation Details

The `secure_hash` function takes a string as input and returns a hexadecimal string representing the hash of the input. The function uses the `Sha256` hasher from the `sha2` crate to compute the hash.

The `main` function demonstrates how to use the `secure_hash` function. It computes the hash of the string `"Bless, Hukporti"` and prints the result to the console.

## Step-by-Step Explanation

Here is a step-by-step explanation of the code:

1. The `secure_hash` function is defined. It takes a string as input and returns a hexadecimal string representing the hash of the input.
2. The function creates a new `Sha256` hasher.
3. The hasher is updated with the bytes of the input string.
4. The hasher is finalized and the result is stored in a variable.
5. The result is formatted as a hexadecimal string and returned.
6. The `main` function is defined. It demonstrates how to use the `secure_hash` function.
7. The `main` function computes the hash of the string `"Bless, Hukporti"` and prints the result to the console.

## Requirements

Step 1: Install Necessary Depandencies
Ensure you have Rust installed. If not, follow the instructions at
```
https://www.rust-lang.org/tools/install
```
## Quickstart

```
git clone https://github.com/Depauli1/Secure_Hash_Function
cd secure_hash_function
```
Compile the Program:
```
cargo build
```
Run the program:
```
cargo run
```
Adjust "Bless, Hukporti" in the main function as needed.

## Contributions

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

The project makes use of the num_bigint and num_traits crates.
Inspired by the need for a concurrent and modular arithmetic solution.

## Author

Bless Hukporti
