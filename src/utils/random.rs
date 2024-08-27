use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn gen_random_string(length: u16) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}