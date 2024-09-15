use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_token(username: &str, user_salt: &str, global_salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", username, user_salt));
    let identifier = format!("{:x}", hasher.finalize());

    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let token_data = format!("{}.{}", identifier, duration);

    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", token_data, global_salt));
    let signature = format!("{:x}", hasher.finalize());

    format!("{}.{}", token_data, signature)
}

fn validate_token(token: &str, global_salt: &str, user_salt: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    let identifier = parts[0];
    let timestamp: u64 = parts[1].parse().unwrap_or(0);
    let signature = parts[2];

    let token_data = format!("{}.{}", identifier, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", token_data, global_salt));
    let expected_signature = format!("{:x}", hasher.finalize());

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    if signature == expected_signature && current_time - timestamp <= 3600 {
        true
    } else {
        false
    }
}


fn main() {
    let username = "user123";
    let user_salt = "unique_user_salt";
    let global_salt = "global_salt_key";

    let token = generate_token(username, user_salt, global_salt);
    println!("Generated Token: {}", token);

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
