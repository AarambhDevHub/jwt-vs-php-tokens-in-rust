# JWT vs PHP-Style Tokens in Rust

## Overview

This repository contains a Rust project that demonstrates secure authentication mechanisms by comparing JSON Web Tokens (JWT) with a classic PHP-style token system. The project focuses on implementing and validating tokens in Rust, exploring security concerns, and handling token expiration. This project is inspired by feedback from @ProGamerru and provides practical code examples and explanations.

## Features

- **JWT Authentication:** Understand the structure and security issues related to JWTs.
- **PHP-Style Tokens:** Implement a classic token system with global and per-user salts.
- **Token Expiration Handling:** Learn how to manage token validity and expiration.
- **Rust Code Examples:** Full implementation in Rust with detailed explanations.

## Installation

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Clone the Repository

    git clone https://github.com/AarambhDevHub/jwt-vs-php-tokens-in-rust.git
    cd jwt-vs-php-tokens-in-rust

## Setup

### Install Dependencies

Navigate to the project directory and run:

    ```
    cargo build
    ```

This will install the necessary dependencies specified in the Cargo.toml file.

## Usage

run:
 
 ```
 cargo run
 ```

## Example

    ```
    fn main() {
        let username = "user123";
        let user_salt = "unique_user_salt";
        let global_salt = "global_salt_key";

        // Generate a token
        let token = generate_token(username, user_salt, global_salt);
        println!("Generated Token: {}", token);

        // Validate the generated token
        let is_valid = validate_token(&token, global_salt, user_salt);
        println!("Is the token valid? {}", is_valid);

        // Extract the identifier and timestamp from the token
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            println!("Invalid token format.");
            return;
        }
        let identifier = parts[0];
        let timestamp: u64 = parts[1].parse().unwrap_or(0);

        // Create an expired token by manipulating the timestamp
        let expired_token = format!("{}x.{}", identifier, timestamp - 3601);
        let is_expired_valid = validate_token(&expired_token, global_salt, user_salt);
        println!("Is the expired token valid? {}", is_expired_valid);
    }
    ```

### License
This project is licensed under the MIT License - see the (LICENSE)[LICENSE] file for details.

Support Us
If you find this project helpful, consider supporting us with a donation. Your contributions help us continue creating valuable content.

(Donate Here)[https://buymeacoffee.com/aarambhdevhub] 💖

Contact
For any questions or feedback, please contact us through GitHub issues or the discussion section.
