use regex::Regex;

pub fn main() {
    // Read two inputs from the prover: a regex pattern and a target string.
    let pattern: String = zkRust::read();
    let target_string: String = zkRust::read();

    // Try to compile the regex pattern. If it fails, write `false` as output and return.
    let regex = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(_) => {
            panic!("Invalid regex pattern");
        }
    };

    // Perform the regex search on the target string.
    let result = regex.is_match(&target_string);

    // Write the result (true or false) to the output.
    zkRust::commit(&result);
}