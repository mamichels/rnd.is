use rand::*;

pub fn generate_number(lower_bound: u32, upper_bound: u32) -> u32 {
    thread_rng().gen_range(lower_bound..upper_bound)
}