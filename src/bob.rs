use regex::Regex;

pub fn reply(message: &str) -> &str {
    if silence(message) {
        "Fine. Be that way!"
    } else if yell(message) && question(message) {
        "Calm down, I know what I'm doing!"
    } else if yell(message) {
        "Whoa, chill out!"
    } else if question(message.trim_end()) {
        "Sure."
    } else {
        "Whatever."
    }
}

pub fn silence(speech: &str) -> bool {
    speech.trim().is_empty()
}

pub fn yell(speech: &str) -> bool {
    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(speech);
    let all_uppercase = speech == speech.to_uppercase();
    has_uppercase && all_uppercase
}

pub fn question(speech: &str) -> bool {
    speech.ends_with('?')
}




[package]
name = "bob"
version = "0.1.0"
edition = "2024"

# Not all libraries from crates.io are available in Exercism's test runner.
# The full list of available libraries is here:
# https://github.com/exercism/rust-test-runner/blob/main/local-registry/Cargo.toml
[dependencies]
regex = "1"