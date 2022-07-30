use rand::*;
use uuid::Uuid;

pub fn generate_number(lower_bound: u32, upper_bound: u32) -> u32 {
    thread_rng().gen_range(lower_bound..upper_bound)
}

pub fn generate_numbers(lower_bound: u32, upper_bound: u32, length: u32) -> Vec<u32> {
    (0..length).map(|_| generate_number(lower_bound, upper_bound)).collect()
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().hyphenated().to_string()
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn generate_uuid_format() {
        let uuid = generate_uuid();
        let re = Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
        assert!(re.is_match(&uuid));
    }
}